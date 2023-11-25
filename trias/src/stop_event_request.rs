use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{RequestPayload, ServiceRequest};

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
    pub dep_arr_time: String,
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
    pub include_realtime_data: bool,
}

impl Default for StopEventParams {
    fn default() -> Self {
        Self {
            number_of_results: 10,
            stop_event_type: "both".to_owned(),
            include_previous_calls: false,
            include_onward_calls: false,
            include_realtime_data: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopEventRequestBuilder {
    requestor_ref: String,
    location_ref: Option<String>,
    dep_arr_time: String,
    params: Option<StopEventParams>,
}

impl StopEventRequestBuilder {
    pub fn new() -> Self {
        let timestamp = Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.%3fZ")
            .to_string();
        Self {
            location_ref: None,
            requestor_ref: "API-Explorer".to_string(),
            dep_arr_time: timestamp,
            params: None,
        }
    }

    pub fn requestor_ref(mut self, requestor_ref: String) -> Self {
        self.requestor_ref = requestor_ref;
        self
    }

    pub fn location_ref(mut self, location_ref: String) -> Self {
        self.location_ref = Some(location_ref);
        self
    }

    pub fn dep_arr_time(mut self, dep_arr_time: String) -> Self {
        self.dep_arr_time = dep_arr_time;
        self
    }

    pub fn params(mut self, params: StopEventParams) -> Self {
        self.params = Some(params);
        self
    }

    pub fn build(self) -> Result<ServiceRequest, &'static str> {
        if self.location_ref.is_none() {
            return Err("Missing required fields");
        }
        Ok(ServiceRequest {
            request_timestamp: self.dep_arr_time.clone(),
            requestor_ref: self.requestor_ref,
            request_payload: RequestPayload::StopEventRequest(StopEventRequest {
                location: Location {
                    location_ref: LocationRef {
                        stop_point_ref: self.location_ref.unwrap(),
                    },
                    dep_arr_time: self.dep_arr_time,
                },
                params: self.params.unwrap_or_default(),
            }),
        })
    }
}

impl Default for StopEventRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
