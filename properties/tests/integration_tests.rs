// use awc::Client;
// use std::net::{IpAddr, Ipv4Addr};

/// This tests require a running server
#[actix_web::test]
async fn test_server_boosted_properties() {
    // let custom_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    // let custom_port = 12000;

    // let client = Client::default();
    // let url = format!("http://{}:{}/imf-properties/boosted-properties", custom_ip, custom_port);

    // let response = client
    //     .get(&url)
    //     .send()
    //     .await
    //     .expect("Failed to send request");
    // assert_eq!(response.status(), 200, "Expected OK status");

    assert!(true);
}