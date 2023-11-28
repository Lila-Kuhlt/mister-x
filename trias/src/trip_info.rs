use serde::{Deserialize, Serialize};

use crate::response::{CallAtStop, DatedJourney, ErrorMessage, GeoPosition};

// request

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TripInfoRequest {
    pub journey_ref: String,
    pub operating_day_ref: String,
    pub params: TripInfoParams,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TripInfoParams {
    pub use_timetabled_data_only: bool,
    pub include_calls: bool,
    pub include_estimated_times: bool,
    pub include_position: bool,
    pub include_service: bool,
    pub include_situation_info: bool,
    pub include_track_sections: bool,
    pub include_track_projection: bool,
}
impl Default for TripInfoParams {
    fn default() -> Self {
        Self {
            use_timetabled_data_only: false,
            include_calls: true,
            include_estimated_times: true,
            include_position: true,
            include_service: true,
            include_situation_info: true,
            include_track_sections: false,
            include_track_projection: false,
        }
    }
}

// response

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TripInfoResponse {
    pub error_message: Option<ErrorMessage>,
    pub trip_info_result: Option<TripInfoResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TripInfoResult {
    #[serde(default)]
    pub previous_call: Vec<CallAtStop>,
    pub current_position: Option<VehiclePosition>,
    #[serde(default)]
    pub onward_call: Vec<CallAtStop>,
    pub service: DatedJourney,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VehiclePosition {
    pub geo_position: GeoPosition,
}
