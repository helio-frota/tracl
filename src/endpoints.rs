mod organization;
pub use organization::*;

// use reqwest::{Response, Url};
// use serde_json::Value;
// use std::{error::Error, path::Path};
// use tokio::fs;
//
// use crate::client::LibClient;
//
// pub async fn upload_dataset(lib: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
//     let mut url = Url::parse(&lib.endpoint("dataset"))?;
//     url.query_pairs_mut().append_pair("labels", labels);
//
//     let file_bytes = fs::read(file_path).await?;
//
//     let response = lib
//         .client()
//         .post(url)
//         .header("Content-Type", "application/octet-stream")
//         .body(file_bytes)
//         .send()
//         .await?;
//
//     Ok(response)
// }
//
// pub async fn upload_sbom(lib: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
//     let mut url = Url::parse(&lib.endpoint("sbom"))?;
//     url.query_pairs_mut().append_pair("labels", labels);
//
//     let file_bytes = fs::read(file_path).await?;
//
//     let response = lib
//         .client()
//         .post(url)
//         .header("Content-Type", "application/octet-stream")
//         .body(file_bytes)
//         .send()
//         .await?;
//
//     Ok(response)
// }
//
// pub async fn upload_advisory(lib: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
//     let mut url = Url::parse(&lib.endpoint("advisory"))?;
//     url.query_pairs_mut().append_pair("labels", labels);
//
//     let file_bytes = fs::read(file_path).await?;
//
//     let response = lib
//         .client()
//         .post(url)
//         .header("Content-Type", "application/octet-stream")
//         .body(file_bytes)
//         .send()
//         .await?;
//
//     Ok(response)
// }
//
// pub async fn organization(lib: &LibClient) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("organization");
//     let resp = lib.client().get(&url).send().await?;
//     Ok(resp)
// }
//
// pub async fn list_weaknesses(lib: &LibClient) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("weakness");
//     let resp = lib.client().get(&url).send().await?;
//     Ok(resp)
// }
//
// pub async fn list_vulnerabilities(lib: &LibClient) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("vulnerability");
//     let resp = lib.client().get(&url).send().await?;
//     Ok(resp)
// }
//
// pub async fn analyze(lib: &LibClient, data: Value) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("vulnerability/analyze");
//     let resp = lib.client().post(&url).json(&data).send().await?;
//     Ok(resp)
// }
//
// pub async fn analysis_component(lib: &LibClient, data: Value) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("analysis/component");
//     let resp = lib.client().get(&url).json(&data).send().await?;
//     Ok(resp)
// }
//
// pub async fn purl_recommend(lib: &LibClient, data: Value) -> Result<Response, Box<dyn Error>> {
//     let url = lib.endpoint("purl/recommend");
//     let resp = lib.client().post(&url).json(&data).send().await?;
//     Ok(resp)
// }
//
// pub async fn info(lib: &LibClient) -> Result<Response, Box<dyn Error>> {
//     let url = lib.custom_endpoint(".well-known/trustify");
//     let resp = lib.client().get(&url).send().await?;
//     Ok(resp)
// }
