use std::path::Path;

use serde_json::{Value, json};
use tracl::{client::LibClient, config::LibConfig, endpoints};

fn api() -> LibClient {
    let config = LibConfig::from_env();
    LibClient::new(config.api_base_url)
}

fn assert_success(res: &reqwest::Response) {
    assert!(res.status().is_success(), "Unexpected status: {}", res.status());
}

#[tokio::test]
async fn upload_dataset() {
    let path = Path::new("tests/data/ds3.zip");
    let res = endpoints::upload_dataset(&api(), "test-label", path)
        .await
        .expect("upload failed");

    assert_eq!(res.status(), 201, "Unexpected status: {}", res.status());
}

#[tokio::test]
async fn upload_sbom() {
    let path = Path::new("tests/data/model-mesh.json");
    let res = endpoints::upload_sbom(&api(), "test-label", path)
        .await
        .expect("upload failed");

    assert_success(&res);
}

#[tokio::test]
async fn upload_advisory() {
    let path = Path::new("tests/data/GHSA-v4q9-qgqf-7jwp.json");
    let res = endpoints::upload_advisory(&api(), "test-label", path)
        .await
        .expect("upload failed");

    assert_success(&res);
}

#[tokio::test]
async fn list_weaknesses() {
    let res = endpoints::list_weaknesses(&api())
        .await
        .expect("list weaknesses failed");

    assert_success(&res);

    let result: Value = res.json().await.expect("Failed to parse JSON");
    // println!("{}", result);
    assert_eq!(result["total"].as_u64(), Some(0), "total mismatch");
}

#[tokio::test]
async fn list_vulnerabilities() {
    let res = endpoints::list_vulnerabilities(&api())
        .await
        .expect("list vulnerabilities failed");

    assert_success(&res);

    let result: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(result["total"].as_u64(), Some(34), "total mismatch");
}

#[tokio::test]
async fn analyze() {
    let data = json!({
        "purls": ["pkg:maven/com.redhat.quarkus.platform/quarkus-bom@2.13.8.Final-redhat-00004"]
    });

    let res = endpoints::analyze(&api(), data).await.expect("analyze failed");

    assert_success(&res);

    let result: Value = res.json().await.expect("Failed to parse JSON");
    let expected = Some("CVE-2023-28867");
    assert_eq!(expected,
        result["pkg:maven/com.redhat.quarkus.platform/quarkus-bom@2.13.8.Final-redhat-00004"][0]["advisories"][0]["document_id"]
                .as_str(),
            "document_id mismatch"
    );
}

#[tokio::test]
async fn info() {
    let res = endpoints::info(&api()).await.expect("info failed");

    assert_success(&res);

    let result: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(result["build"]["target"]["os"].as_str(), Some("linux"), "OS mismatch");
}
