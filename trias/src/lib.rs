use std::error::Error;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_xml_rs::to_string;

use location_information::{Location, LocationInformationRequest};
use response::{DeliveryPayload, TriasResponse};
use stop_event::{StopEventRequest, StopEventResponse};
use trip_info::{TripInfoParams, TripInfoRequest, TripInfoResult};

pub use location_information::LocationInformationRequestBuilder;
pub use stop_event::StopEventRequestBuilder;

mod location_information;
pub mod response;
mod stop_event;
mod trip_info;

#[derive(Debug, Serialize)]
pub struct ServiceRequest {
    #[serde(rename = "siri:RequestTimestamp")]
    pub request_timestamp: DateTime<Utc>,
    #[serde(rename = "siri:RequestorRef")]
    pub requestor_ref: String,
    #[serde(rename = "RequestPayload")]
    pub request_payload: RequestPayload,
}

#[derive(Debug, Serialize)]
pub enum RequestPayload {
    LocationInformationRequest(LocationInformationRequest),
    StopEventRequest(StopEventRequest),
    TripInfoRequest(TripInfoRequest),
}

pub fn generate_service_request(
    access_token: String,
    payload: RequestPayload,
) -> Result<String, &'static str> {
    let request = ServiceRequest {
        request_timestamp: Utc::now(),
        requestor_ref: access_token,
        request_payload: payload,
    };
    let xml_string = to_string(&request).map_err(|_| "Failed to serialize to XML")?;

    let xml_string = xml_string.replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, "");
    Ok(format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<Trias version="1.2" xmlns="http://www.vdv.de/trias" xmlns:siri="http://www.siri.org.uk/siri"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.vdv.de/trias
file:///C:/development/HEAD/extras/TRIAS/TRIAS_1.2/Trias.xsd">
     {}
</Trias>"#,
        xml_string
    ))
}

pub async fn post_request(
    api_endpoint: &str,
    request: &str,
) -> Result<TriasResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(api_endpoint)
        .header(reqwest::header::CONTENT_TYPE, "application/xml")
        .body(request.to_owned())
        .send()
        .await?
        .text()
        .await?;

    let deserialized: TriasResponse = serde_xml_rs::from_str(&response)?;
    Ok(deserialized)
}

pub async fn search_stops(
    stop_id: String,
    access_token: String,
    api_endpoint: &str,
    number_of_results: u32,
) -> Result<Vec<Location>, Box<dyn Error>> {
    let payload = LocationInformationRequestBuilder::new(stop_id)
        .number_of_results(number_of_results)
        .include_pt_modes(false)
        .build();

    let xml_request = generate_service_request(access_token, payload)?;
    let response = post_request(api_endpoint, &xml_request).await?;

    let DeliveryPayload::LocationInformationResponse(response) =
        response.service_delivery.delivery_payload
    else {
        panic!("Wrong response type");
    };

    let locations = response
        .location_result
        .into_iter()
        .map(|x| x.location)
        .collect();
    Ok(locations)
}

pub async fn stop_events(
    location_ref: String,
    access_token: String,
    number_of_results: u32,
    api_endpoint: &str,
) -> Result<StopEventResponse, Box<dyn Error>> {
    let params = stop_event::StopEventParams {
        number_of_results,
        include_realtime_data: true,
        include_previous_calls: true,
        include_onward_calls: true,
        ..Default::default()
    };
    let payload = StopEventRequestBuilder::new(location_ref)
        .params(params)
        .build();

    let xml_request = generate_service_request(access_token, payload)?;
    let response = post_request(api_endpoint, &xml_request).await?;

    let DeliveryPayload::StopEventResponse(response) = response.service_delivery.delivery_payload
    else {
        panic!("Wrong response type");
    };
    Ok(response)
}

pub async fn trip_info(
    journey_ref: String,
    operating_day_ref: String,
    access_token: String,
    api_endpoint: &str,
) -> Result<TripInfoResult, Box<dyn Error>> {
    let payload = RequestPayload::TripInfoRequest(TripInfoRequest {
        journey_ref,
        operating_day_ref,
        params: TripInfoParams::default(),
    });

    let xml_request = generate_service_request(access_token, payload)?;
    let response = post_request(api_endpoint, &xml_request).await?;

    let DeliveryPayload::TripInfoResponse(response) = response.service_delivery.delivery_payload
    else {
        panic!("Wrong response type");
    };

    response.trip_info_result.ok_or_else(|| {
        response
            .error_message
            .map(|err| err.text.text)
            .unwrap_or_default()
            .into()
    })
}
