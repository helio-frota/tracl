// use std::path::Path;

// use serde_json::{Value, json};
use tracl::client::LibClient;

fn client() -> LibClient {
    // std::env::set_var("API_TOKEN", "abc");
    LibClient::new("http://localhost:8080")
}

// fn assert_success(res: &reqwest::Response) {
//     assert!(res.status().is_success(), "Unexpected status: {}", res.status());
// }

#[tokio::test]
async fn organization() {
    let result = client().organization().get_all().await.unwrap();
    dbg!("{result}");
    assert_eq!(result.total, 11);
}

#[tokio::test]
async fn organization2() {
    let result = client().organization().get_all().await.unwrap();
    println!("{:?}", result);
    assert_eq!(result.total, 11);
}

// #[tokio::test]
// async fn upload_dataset() {
//     let path = Path::new("tests/data/ds3.zip");
//     let res = endpoints::upload_dataset(&lib(), "test-label", path)
//         .await
//         .expect("upload failed");
//
//     assert_eq!(res.status(), 201, "Unexpected status: {}", res.status());
// }
//
// #[tokio::test]
// async fn upload_sbom() {
//     let path = Path::new("tests/data/model-mesh.json");
//     let res = endpoints::upload_sbom(&lib(), "test-label", path)
//         .await
//         .expect("upload failed");
//
//     assert_success(&res);
// }
//
// #[tokio::test]
// async fn upload_advisory() {
//     let path = Path::new("tests/data/GHSA-v4q9-qgqf-7jwp.json");
//     let res = endpoints::upload_advisory(&lib(), "test-label", path)
//         .await
//         .expect("upload failed");
//
//     assert_success(&res);
// }
//
// #[tokio::test]
// async fn organization() {
//     let res = endpoints::organization(&lib()).await.expect("failed");
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     assert_eq!(result["total"].as_u64(), Some(0), "foo bar");
// }
//
// #[tokio::test]
// async fn list_weaknesses() {
//     let res = endpoints::list_weaknesses(&lib())
//         .await
//         .expect("list weaknesses failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     // println!("{}", result);
//     assert_eq!(result["total"].as_u64(), Some(0), "total mismatch");
// }
//
// #[tokio::test]
// #[ignore]
// async fn list_vulnerabilities() {
//     let res = endpoints::list_vulnerabilities(&lib())
//         .await
//         .expect("list vulnerabilities failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     assert_eq!(result["total"].as_u64(), Some(34), "total mismatch");
// }
//
// #[tokio::test]
// async fn analyze() {
//     let data = json!({
//         "purls": ["pkg:maven/aopalliance"]
//     });
//
//     let res = endpoints::analyze(&lib(), data).await.expect("failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     let expected = Some("CVE-2023-28867");
//     assert_eq!(expected, result.as_str());
// }
//
// #[tokio::test]
// async fn analysis_component() {
//     let data = json!({
//         "purls": ["pkg:maven/aopalliance"]
//     });
//
//     let res = endpoints::analysis_component(&lib(), data).await.expect("failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     let expected = Some("CVE-2023-28867");
//     assert_eq!(expected, result.as_str());
// }
//
// #[tokio::test]
// async fn purl_recommend() {
//     let data = json!({
//         "purls": ["pkg:maven/com.redhat.quarkus.platform/quarkus-bom@2.13.8.Final-redhat-00004"]
//     });
//
//     let res = endpoints::purl_recommend(&lib(), data).await.expect("failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     let expected = Some("FOO");
//     assert_eq!(expected, result.as_str());
// }
//
// #[tokio::test]
// async fn info() {
//     let res = endpoints::info(&lib()).await.expect("info failed");
//
//     assert_success(&res);
//
//     let result: Value = res.json().await.expect("Failed to parse JSON");
//     assert_eq!(result["build"]["target"]["os"].as_str(), Some("linux"), "OS mismatch");
// }
