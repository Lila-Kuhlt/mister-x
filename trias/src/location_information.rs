use serde::{Deserialize, Serialize};

use crate::response::{ErrorMessage, GeoPosition, Text};
use crate::RequestPayload;

// request

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

pub struct LocationInformationRequestBuilder {
    location_name: String,
    location_type: String,
    number_of_results: u32,
    include_pt_modes: bool,
}

impl LocationInformationRequestBuilder {
    pub fn new(location_name: String) -> Self {
        LocationInformationRequestBuilder {
            location_name,
            location_type: "stop".to_owned(),
            number_of_results: 2,
            include_pt_modes: false,
        }
    }

    pub fn location_name(mut self, location_name: String) -> Self {
        self.location_name = location_name;
        self
    }

    pub fn location_type(mut self, location_type: &str) -> Self {
        self.location_type = location_type.to_string();
        self
    }

    pub fn number_of_results(mut self, number_of_results: u32) -> Self {
        self.number_of_results = number_of_results;
        self
    }

    pub fn include_pt_modes(mut self, include_pt_modes: bool) -> Self {
        self.include_pt_modes = include_pt_modes;
        self
    }

    pub fn build(self) -> RequestPayload {
        RequestPayload::LocationInformationRequest(LocationInformationRequest {
            initial_input: InitialInput {
                location_name: self.location_name,
            },
            restrictions: Restrictions {
                location_type: self.location_type,
                number_of_results: self.number_of_results,
                include_pt_modes: self.include_pt_modes,
            },
        })
    }
}

// response

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInformationResponse {
    pub error_message: Option<ErrorMessage>,
    #[serde(default)]
    pub location_result: Vec<LocationResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationResult {
    pub location: Location,
    pub complete: bool,
    pub probability: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub stop_point: StopPoint,
    pub location_name: Text,
    pub geo_position: GeoPosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopPoint {
    pub stop_point_ref: String,
    pub stop_point_name: Text,
    pub locality_ref: String,
    pub wheelchair_accessible: bool,
    pub lighting: bool,
    pub covered: bool,
}
