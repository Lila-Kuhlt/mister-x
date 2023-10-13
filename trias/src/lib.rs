mod location_information;
pub mod response;
mod stop_event_request;

pub use location_information::LocationInformationRequestBuilder;
use location_information::{LocationInformationRequest, LocationResult};
use response::TriasResponse;
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
file:///C:/development/HEAD/extras/TRIAS/TRIAS_1.1/Trias.xsd">
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
    println!("Request: {}", request);
    let response = client
        .post(api_endpoint)
        .header("Content-Type", "application/xml")
        .body(request.to_string())
        .send()
        .await?
        .text()
        .await?;
    println!("{}", &response);

    let deserialized: TriasResponse = serde_xml_rs::from_str(&response)?;
    println!("{:?}", deserialized);
    Ok(deserialized)
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
