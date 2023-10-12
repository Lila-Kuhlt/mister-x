#[tokio::test]
async fn test_fetch_location_information() {
    dotenv::dotenv().ok();
    let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
    let access_token = std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");

    let builder = trias::LocationInformationRequestBuilder::new()
        .location_name("Karlsruhe Hauptbahnhof")
        .requestor_ref(&access_token)
        .number_of_results(5)
        .include_pt_modes(true)
        .build()
        //... set other fields ...
        ;

    let xml_request = trias::generate_service_request(builder).unwrap();
    dbg!(&xml_request);
    let response = trias::fetch_location_information(api_endpoint, &xml_request)
        .await
        .unwrap();

    assert!(
        response
            .service_delivery
            .delivery_payload
            .location_information_response
            .len()
            > 0
    );
}
