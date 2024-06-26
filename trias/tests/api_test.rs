use trias::response::DeliveryPayload;

#[tokio::test]
async fn test_fetch_location_information() {
    dotenv::dotenv().ok();
    let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
    let access_token = std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");

    let payload = trias::LocationInformationRequestBuilder::new("Karlsruhe Hauptbahnhof".to_owned())
        .number_of_results(2)
        .include_pt_modes(false)
        .build()
        //... set other fields ...
        ;

    let xml_request = trias::generate_service_request(access_token, payload).unwrap();
    let response = trias::post_request(api_endpoint, &xml_request).await.unwrap();

    let DeliveryPayload::LocationInformationResponse(response) = response.service_delivery.delivery_payload else {
        panic!("Wrong response type");
    };
    for stop in &response.location_result {
        println!("{:?}", stop);
    }
    assert!(response.location_result.len() > 0);
}

#[tokio::test]
async fn test_fetch_stop_event() {
    dotenv::dotenv().ok();
    let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
    let access_token = std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");

    let payload = trias::StopEventRequestBuilder::new("de:08212:7".to_owned())
        .dep_arr_time("2023-10-13T00:24:28".to_owned())
        .build()
        //... set other fields ...
        ;

    let xml_request = trias::generate_service_request(access_token, payload).unwrap();
    println!("{}", xml_request);
    let response = trias::post_request(api_endpoint, &xml_request).await.unwrap();

    let DeliveryPayload::StopEventResponse(response) = response.service_delivery.delivery_payload else {
        panic!("Wrong response type");
    };

    let result = &response[0].stop_event_result;
    assert!(result[0].stop_event.this_call.call_at_stop.service_departure.is_some());
}
