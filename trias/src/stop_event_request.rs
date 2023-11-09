// src/stop_event_request.rs

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
            stop_event_type: "departure".to_string(),
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
    dep_arr_time: Option<String>,
    params: Option<StopEventParams>,
}

impl StopEventRequestBuilder {
    pub fn new() -> Self {
        let timestamp = Local::now()
            .with_timezone(&chrono_tz::Europe::Berlin)
            .format("%Y-%m-%dT%H:%M:%S.%3fZ")
            .to_string();
        Self {
            location_ref: None,
            requestor_ref: "API-Explorer".to_string(),
            dep_arr_time: Some(timestamp),
            params: None,
        }
    }

    pub fn requestor_ref(&mut self, requestor_ref: String) -> &mut Self {
        self.requestor_ref = requestor_ref;
        self
    }

    pub fn location_ref(&mut self, location_ref: String) -> &mut Self {
        self.location_ref = Some(location_ref);
        self
    }

    pub fn dep_arr_time(&mut self, dep_arr_time: String) -> &mut Self {
        self.dep_arr_time = Some(dep_arr_time);
        self
    }

    pub fn params(&mut self, params: StopEventParams) -> &mut Self {
        self.params = Some(params);
        self
    }

    pub fn build(&self) -> Result<ServiceRequest, &'static str> {
        if self.location_ref.is_none() || self.dep_arr_time.is_none() {
            return Err("Missing required fields");
        }
        Ok(ServiceRequest {
            request_timestamp: self.dep_arr_time.clone().unwrap(),
            requestor_ref: self.requestor_ref.clone(),
            request_payload: RequestPayload::StopEventRequest(StopEventRequest {
                location: Location {
                    location_ref: LocationRef {
                        stop_point_ref: self.location_ref.clone().unwrap(),
                    },
                    dep_arr_time: self.dep_arr_time.clone().unwrap(),
                },
                params: self.params.clone().unwrap_or_default(),
            }),
        })
    }
}

impl Default for StopEventRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// src/stop_event_response.rs

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{RequestPayload, ServiceRequest};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResult {
    pub result_id: Option<String>,
    pub stop_event: Option<StopEventDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventDetails {
    pub this_call: ThisCall,
    pub service: ServiceDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThisCall {
    pub call_at_stop: CallAtStopDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CallAtStopDetails {
    pub stop_point_ref: String,
    pub stop_point_name: TextLang,
    pub planned_bay: TextLang,
    pub service_departure: ServiceDepartureDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDepartureDetails {
    pub timetabled_time: String,
    pub estimated_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDetails {
    pub operating_day_ref: String,
    pub journey_ref: String,
    // ... other fields like LineRef, DirectionRef, Mode, etc. ...
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextLang {
    pub text: String,
    pub language: String,
}

// You can expand upon these structs if the API has more detailed responses or other nested elements.
