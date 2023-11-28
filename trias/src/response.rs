use serde::{Deserialize, Serialize};

use crate::location_information::LocationInformationResponse;
use crate::stop_event::StopEventResponse;
use crate::trip_info::TripInfoResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TriasResponse {
    pub service_delivery: ServiceDelivery,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDelivery {
    pub response_timestamp: String,
    pub producer_ref: String,
    pub status: bool,
    pub more_data: String,
    pub language: Language,
    pub calc_time: Option<String>,
    pub delivery_payload: DeliveryPayload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeliveryPayload {
    LocationInformationResponse(LocationInformationResponse),
    StopEventResponse(Vec<StopEventResponse>),
    TripInfoResponse(TripInfoResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorMessage {
    pub code: String,
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Text {
    pub text: String,
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Language {
    #[serde(rename = "de")]
    De,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CallAtStop {
    pub stop_point_ref: String,
    pub stop_point_name: Text,
    pub planned_bay: Option<Text>,
    pub service_arrival: Option<Service>,
    pub service_departure: Option<Service>,
    pub stop_seq_number: String,
    pub demand_stop: bool,
    pub unplanned_stop: bool,
    pub not_serviced_stop: bool,
    pub no_boarding_at_stop: bool,
    pub no_alighting_at_stop: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    pub timetabled_time: String,
    pub estimated_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DatedJourney {
    pub operating_day_ref: String,
    pub journey_ref: String,
    pub service_section: ServiceSection,
    #[serde(default)]
    pub attribute: Vec<Attribute>,
    pub origin_stop_point_ref: String,
    pub origin_text: Text,
    pub destination_text: Text,
    pub unplanned: bool,
    pub cancelled: bool,
    pub deviation: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSection {
    pub line_ref: String,
    pub direction_ref: String,
    pub mode: Mode,
    pub published_line_name: Text,
    pub operator_ref: String,
    pub route_description: Option<Text>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    pt_mode: String,
    tram_submode: Option<String>,
    name: Text,
    rail_submode: Option<String>,
    bus_submode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub text: Text,
    pub code: String,
    pub mandatory: bool,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoPosition {
    pub longitude: f64,
    pub latitude: f64,
}
