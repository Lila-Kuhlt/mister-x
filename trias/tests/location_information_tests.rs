// tests/location_information_test.rs

#[cfg(test)]
mod tests {
    use trias::generate_service_request;
    use trias::LocationInformationRequestBuilder;
    use trias::RequestPayload;

    #[test]
    fn test_location_information_request_builder() {
        let payload = LocationInformationRequestBuilder::new("Bern".to_owned())
            .number_of_results(5)
            .include_pt_modes(true)
            .build();

        if let RequestPayload::LocationInformationRequest(request) = payload {
            assert_eq!(request.initial_input.location_name, "Bern");
            assert_eq!(request.restrictions.location_type, "stop");
            assert_eq!(request.restrictions.number_of_results, 5);
            assert!(request.restrictions.include_pt_modes);
        } else {
            panic!("Wrong request type");
        }
    }

    #[test]
    fn test_location_information_request_serialization() {
        let payload = LocationInformationRequestBuilder::new("Karlsruhe Hauptbahnhof".to_owned()).build();

        let xml_output = generate_service_request("API-Explorer".to_owned(), payload).unwrap();

        // assert the xml_output against the expected XML string
        // or use it to perform an actual API request
    }
}
