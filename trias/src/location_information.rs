// src/location_information.rs

use serde::{Deserialize, Serialize};

use chrono::Utc;

use crate::{RequestPayload, ServiceRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInformationRequest {
    #[serde(rename = "InitialInput")]
    pub initial_input: InitialInput,
    #[serde(rename = "Restrictions")]
    pub restrictions: Restrictions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialInput {
    #[serde(rename = "LocationName")]
    pub location_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Restrictions {
    #[serde(rename = "Type")]
    pub location_type: String, // Using location_type to avoid Rust's reserved word 'type'
    #[serde(rename = "NumberOfResults")]
    pub number_of_results: u32,
    #[serde(rename = "IncludePtModes")]
    pub include_pt_modes: bool,
}
// src/location_request_builder.rs

pub struct LocationInformationRequestBuilder {
    requestor_ref: String,
    location_name: String,
    location_type: String,
    number_of_results: u32,
    include_pt_modes: bool,
}

impl LocationInformationRequestBuilder {
    pub fn new() -> Self {
        LocationInformationRequestBuilder {
            requestor_ref: "API-Explorer".to_string(),
            location_name: "Bern".to_string(),
            location_type: "stop".to_string(),
            number_of_results: 2,
            include_pt_modes: true,
        }
    }

    pub fn requestor_ref(&mut self, requestor_ref: &str) -> &mut Self {
        self.requestor_ref = requestor_ref.to_string();
        self
    }

    pub fn location_name(&mut self, location_name: &str) -> &mut Self {
        self.location_name = location_name.to_string();
        self
    }

    pub fn location_type(&mut self, location_type: &str) -> &mut Self {
        self.location_type = location_type.to_string();
        self
    }

    pub fn number_of_results(&mut self, number_of_results: u32) -> &mut Self {
        self.number_of_results = number_of_results;
        self
    }

    pub fn include_pt_modes(&mut self, include_pt_modes: bool) -> &mut Self {
        self.include_pt_modes = include_pt_modes;
        self
    }

    pub fn build(&self) -> ServiceRequest {
        ServiceRequest {
            request_timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string(),
            requestor_ref: self.requestor_ref.clone(),
            request_payload: RequestPayload::LocationInformationRequest(
                LocationInformationRequest {
                    initial_input: InitialInput {
                        location_name: self.location_name.clone(),
                    },
                    restrictions: Restrictions {
                        location_type: self.location_type.clone(),
                        number_of_results: self.number_of_results,
                        include_pt_modes: self.include_pt_modes,
                    },
                },
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInformationResponse {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDelivery,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDelivery {
    #[serde(rename = "ResponseTimestamp")]
    pub response_timestamp: String,
    #[serde(rename = "ProducerRef")]
    pub producer_ref: String,
    #[serde(rename = "Status")]
    pub status: bool,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "CalcTime")]
    pub calc_time: Option<u32>,
    #[serde(rename = "DeliveryPayload")]
    pub delivery_payload: DeliveryPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPayload {
    #[serde(rename = "LocationInformationResponse")]
    pub location_information_response: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "StopPoint")]
    pub stop_point: Option<StopPoint>,
    #[serde(rename = "LocationName")]
    pub location_name: Option<TextLang>,
    #[serde(rename = "GeoPosition")]
    pub geo_position: Option<GeoPosition>,
    #[serde(rename = "Complete")]
    pub complete: Option<bool>,
    #[serde(rename = "Probability")]
    pub probability: Option<f32>,
    #[serde(rename = "Mode")]
    pub mode: Option<Mode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopPoint {
    #[serde(rename = "StopPointRef")]
    pub stop_point_ref: String,
    #[serde(rename = "StopPointName")]
    pub stop_point_name: TextLang,
    #[serde(rename = "LocalityRef")]
    pub locality_ref: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextLang {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Language")]
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoPosition {
    #[serde(rename = "Longitude")]
    pub longitude: f32,
    #[serde(rename = "Latitude")]
    pub latitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    #[serde(rename = "PtMode")]
    pub pt_mode: String,
    #[serde(rename = "RailSubmode", default)]
    pub rail_submode: Option<String>,
    #[serde(rename = "FunicularSubmode", default)]
    pub funicular_submode: Option<String>,
}
