// tests/stop_event_request_test.rs

#[cfg(test)]
mod tests {
    use trias::{generate_service_request, RequestPayload, StopEventRequestBuilder}; // Adjust this import based on your library setup

    #[test]
    fn test_stop_event_request_builder() {
        let request = StopEventRequestBuilder::new()
            .location_ref("8507000".to_string())
            .dep_arr_time("2023-10-11T11:24:28".to_string())
            .build()
            .unwrap();

        if let RequestPayload::StopEventRequest(request) = request.request_payload {
            assert_eq!(request.location.location_ref.stop_point_ref, "8507000");
            assert_eq!(request.location.dep_arr_time, "2023-10-11T11:24:28");
        } else {
            panic!("Wrong request type");
        }
    }

    #[test]
    fn test_stop_event_request_builder_missing_fields() {
        let request = StopEventRequestBuilder::new().build();
        assert!(request.is_err());
    }

    #[test]
    fn test_stop_event_request_serialization() {
        let builder = StopEventRequestBuilder::new()
            .location_ref("8507000".to_string())
            .dep_arr_time("2023-10-11T11:24:28".to_string())
            .build()
            .unwrap();

        let xml_output = generate_service_request(builder).unwrap();

        // assert the xml_output against the expected XML string
        // or use it to perform an actual API request
    }
}
