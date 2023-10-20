mod location_information;
pub mod response;
mod stop_event_request;

use chrono::Utc;
pub use location_information::LocationInformationRequestBuilder;
use location_information::{LocationInformationRequest, LocationResult};
use response::{DeliveryPayload, Location, StopEventResponse, TriasResponse};
pub use stop_event_request::StopEventRequestBuilder;

use serde_xml_rs::to_string;

pub fn generate_service_request(builder: ServiceRequest) -> Result<String, &'static str> {
    let xml_string = to_string(&builder).map_err(|_| "Failed to serialize to XML")?;

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

use std::error::Error;

pub async fn post_request(
    api_endpoint: &str,
    request: &str,
) -> Result<TriasResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    //println!("Request: {}", request);
    let response = client
        .post(api_endpoint)
        .header("Content-Type", "application/xml")
        .body(request.to_string())
        .send()
        .await?
        .text()
        .await?;
    //println!("{}", &response);

    let deserialized: TriasResponse = serde_xml_rs::from_str(&response)?;
    //println!("{:?}", deserialized);
    Ok(deserialized)
}

pub async fn search_stops(
    name: String,
    access_token: String,
    api_endpoint: &str,
    number_of_results: u32,
) -> Result<Vec<Location>, Box<dyn Error>> {
    let builder = LocationInformationRequestBuilder::new()
        .location_name(name)
        .requestor_ref(access_token)
        .number_of_results(number_of_results)
        .include_pt_modes(false)
        .build();

    let xml_request = generate_service_request(builder).unwrap();
    let response = post_request(api_endpoint, &xml_request).await.unwrap();

    let DeliveryPayload::LocationInformationResponse(response) =
        response.service_delivery.ok_or("No service_delivery")?.delivery_payload
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
) -> Result<Vec<StopEventResponse>, Box<dyn Error>> {
    let params = stop_event_request::StopEventParams {
        number_of_results,
        include_realtime_data: true,
        include_previous_calls: true,
        include_onward_calls: true,
        ..Default::default()
    };
    let builder = StopEventRequestBuilder::new()
        .location_ref(location_ref)
        .requestor_ref(access_token)
        .params(params)
        .build()
        .unwrap()
        //... set other fields ...
        ;

    let xml_request = generate_service_request(builder)?;
    let response = post_request(api_endpoint, &xml_request).await?;

    let DeliveryPayload::StopEventResponse(response) = response.service_delivery.ok_or("no service delivery")?.delivery_payload
    else {
        panic!("Wrong response type");
    };
    Ok(response)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRequest {
    #[serde(rename = "siri:RequestTimestamp")]
    pub request_timestamp: String, // Will use this to store the current timestamp
    #[serde(rename = "siri:RequestorRef")]
    pub requestor_ref: String,
    #[serde(rename = "RequestPayload")]
    pub request_payload: RequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestPayload {
    LocationInformationRequest(LocationInformationRequest),
    StopEventRequest(stop_event_request::StopEventRequest),
}
