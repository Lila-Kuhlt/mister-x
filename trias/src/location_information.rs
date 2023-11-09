// src/location_information.rs

use serde::{Deserialize, Serialize};

use chrono::Utc;

use crate::{RequestPayload, ServiceRequest};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInformationRequest {
    pub initial_input: InitialInput,
    pub restrictions: Restrictions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitialInput {
    pub location_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Restrictions {
    #[serde(rename = "Type")]
    pub location_type: String, // Using location_type to avoid Rust's reserved word 'type'
    pub number_of_results: u32,
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
            location_name: "Karlsruhe Hauptbahnhof".to_string(),
            location_type: "stop".to_string(),
            number_of_results: 2,
            include_pt_modes: false,
        }
    }

    pub fn requestor_ref(&mut self, requestor_ref: String) -> &mut Self {
        self.requestor_ref = requestor_ref;
        self
    }

    pub fn location_name(&mut self, location_name: String) -> &mut Self {
        self.location_name = location_name;
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationResult {
    location: Location,
    complete: String,
    probability: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub stop_point: StopPoint,
    pub location_name: TextLang,
    pub geo_position: GeoPosition,
    pub complete: bool,
    pub probability: f32,
    pub mode: Mode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopPoint {
    pub stop_point_ref: String,
    pub stop_point_name: TextLang,
    pub locality_ref: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextLang {
    pub text: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoPosition {
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    pub pt_mode: String,
    #[serde(default)]
    pub rail_submode: Option<String>,
    #[serde(default)]
    pub funicular_submode: Option<String>,
}
