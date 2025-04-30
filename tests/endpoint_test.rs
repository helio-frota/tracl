use serde_json::Value;
use tracl::{client::LibClient, config::LibConfig, endpoints};

fn client() -> LibClient {
    let config = LibConfig::from_env();
    LibClient::new(config.api_base_url)
}

#[tokio::test]
async fn test_info() {
    let res = endpoints::info(&client()).await.expect("info failed");
    assert!(
        res.status().is_success(),
        "Unexpected status: {}",
        res.status()
    );

    // let body = res.text().await.expect("Failed to read body");
    // assert!(!body.is_empty(), "Response body is empty");

    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(
        body["build"]["target"]["os"].as_str(),
        Some("linux"),
        "OS mismatch"
    );
    println!("Body: {}", body);
}
