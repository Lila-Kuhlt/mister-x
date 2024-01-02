use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::response::{CallAtStop, DatedJourney, ErrorMessage};
use crate::RequestPayload;

// request

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventRequest {
    pub location: Location,
    pub params: StopEventParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub location_ref: LocationRef,
    pub dep_arr_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationRef {
    pub stop_point_ref: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventParams {
    pub number_of_results: u32,
    pub stop_event_type: String,
    pub include_previous_calls: bool,
    pub include_onward_calls: bool,
    pub include_operating_days: bool,
    pub include_realtime_data: bool,
}

impl Default for StopEventParams {
    fn default() -> Self {
        Self {
            number_of_results: 10,
            stop_event_type: "both".to_owned(),
            include_previous_calls: false,
            include_onward_calls: false,
            include_operating_days: false,
            include_realtime_data: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopEventRequestBuilder {
    location_ref: String,
    dep_arr_time: DateTime<Utc>,
    params: Option<StopEventParams>,
}

impl StopEventRequestBuilder {
    pub fn new(location_ref: String) -> Self {
        let timestamp = Utc::now();
        Self {
            location_ref,
            dep_arr_time: timestamp,
            params: None,
        }
    }

    pub fn location_ref(mut self, location_ref: String) -> Self {
        self.location_ref = location_ref;
        self
    }

    pub fn dep_arr_time(mut self, dep_arr_time: DateTime<Utc>) -> Self {
        self.dep_arr_time = dep_arr_time;
        self
    }

    pub fn params(mut self, params: StopEventParams) -> Self {
        self.params = Some(params);
        self
    }

    pub fn build(self) -> RequestPayload {
        RequestPayload::StopEventRequest(StopEventRequest {
            location: Location {
                location_ref: LocationRef {
                    stop_point_ref: self.location_ref,
                },
                dep_arr_time: self.dep_arr_time,
            },
            params: self.params.unwrap_or_default(),
        })
    }
}

// response

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResponse {
    pub error_message: Option<ErrorMessage>,
    #[serde(default)]
    pub stop_event_result: Vec<StopEventResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResult {
    pub result_id: String,
    pub stop_event: StopEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEvent {
    #[serde(default)]
    pub previous_call: Vec<Call>,
    pub this_call: Call,
    #[serde(default)]
    pub onward_call: Vec<Call>,
    pub service: DatedJourney,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Call {
    pub call_at_stop: CallAtStop,
}
