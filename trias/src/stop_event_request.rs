// src/stop_event_request.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopEventRequest {
    pub location_ref: String,
    pub dep_arr_time: String,
    pub params: StopEventParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopEventParams {
    pub number_of_results: usize,
    pub stop_event_type: String,
    pub include_previous_calls: bool,
    pub include_onward_calls: bool,
    pub include_realtime_data: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopEventRequestBuilder {
    location_ref: Option<String>,
    dep_arr_time: Option<String>,
    params: Option<StopEventParams>,
}

impl StopEventRequestBuilder {
    pub fn new() -> Self {
        Self {
            location_ref: None,
            dep_arr_time: None,
            params: None,
        }
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

    pub fn build(&self) -> Result<StopEventRequest, &'static str> {
        if self.location_ref.is_none() || self.dep_arr_time.is_none() {
            return Err("Missing required fields");
        }

        Ok(StopEventRequest {
            location_ref: self.location_ref.clone().unwrap(),
            dep_arr_time: self.dep_arr_time.clone().unwrap(),
            params: self.params.clone().unwrap_or_else(|| StopEventParams {
                number_of_results: 1,
                stop_event_type: "departure".to_string(),
                include_previous_calls: false,
                include_onward_calls: false,
                include_realtime_data: false,
            }),
        })
    }
}

// src/stop_event_response.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StopEventResponse {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDeliveryStopEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDeliveryStopEvent {
    #[serde(rename = "ResponseTimestamp")]
    pub response_timestamp: String,
    #[serde(rename = "ProducerRef")]
    pub producer_ref: String,
    #[serde(rename = "Status")]
    pub status: bool,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "CalcTime")]
    pub calc_time: u32,
    #[serde(rename = "DeliveryPayload")]
    pub delivery_payload: DeliveryPayloadStopEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPayloadStopEvent {
    #[serde(rename = "StopEventResponse")]
    pub stop_event_response: Vec<StopEventResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopEventResult {
    #[serde(rename = "ResultId")]
    pub result_id: String,
    #[serde(rename = "StopEvent")]
    pub stop_event: StopEventDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopEventDetails {
    #[serde(rename = "ThisCall")]
    pub this_call: ThisCall,
    #[serde(rename = "Service")]
    pub service: ServiceDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThisCall {
    #[serde(rename = "CallAtStop")]
    pub call_at_stop: CallAtStopDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallAtStopDetails {
    #[serde(rename = "StopPointRef")]
    pub stop_point_ref: String,
    #[serde(rename = "StopPointName")]
    pub stop_point_name: TextLang,
    #[serde(rename = "PlannedBay")]
    pub planned_bay: TextLang,
    #[serde(rename = "ServiceDeparture")]
    pub service_departure: ServiceDepartureDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDepartureDetails {
    #[serde(rename = "TimetabledTime")]
    pub timetabled_time: String,
    #[serde(rename = "EstimatedTime")]
    pub estimated_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDetails {
    #[serde(rename = "OperatingDayRef")]
    pub operating_day_ref: String,
    #[serde(rename = "JourneyRef")]
    pub journey_ref: String,
    // ... other fields like LineRef, DirectionRef, Mode, etc. ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextLang {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Language")]
    pub language: String,
}

// You can expand upon these structs if the API has more detailed responses or other nested elements.
