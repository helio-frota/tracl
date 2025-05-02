use serde_json::{Value, json};
use tracl::{client::LibClient, config::LibConfig, endpoints};

fn client() -> LibClient {
    let config = LibConfig::from_env();
    LibClient::new(config.api_base_url)
}

#[tokio::test]
async fn analyze() {
    let data = json!({
        "purls": ["pkg:npm/accepts@1.3.8"]
    });

    let res = endpoints::analyze(&client(), data)
        .await
        .expect("analyze failed");

    assert!(
        res.status().is_success(),
        "Unexpected status: {}",
        res.status()
    );

    let result: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(result, serde_json::json!({}), "Empty JSON expected");
    println!("Result: {}", result);
}

#[tokio::test]
async fn info() {
    let res = endpoints::info(&client()).await.expect("info failed");
    assert!(
        res.status().is_success(),
        "Unexpected status: {}",
        res.status()
    );

    let result: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(
        result["build"]["target"]["os"].as_str(),
        Some("linux"),
        "OS mismatch"
    );
}
