use reqwest::{Response, Url};
use serde_json::Value;
use std::{error::Error, path::Path};
use tokio::fs;

use crate::client::LibClient;

pub async fn upload_dataset(api: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
    let mut url = Url::parse(&api.endpoint("dataset"))?;
    url.query_pairs_mut().append_pair("labels", labels);

    let file_bytes = fs::read(file_path).await?;

    let response = api
        .client()
        .post(url)
        .header("Content-Type", "application/octet-stream")
        .body(file_bytes)
        .send()
        .await?;

    Ok(response)
}

pub async fn upload_sbom(api: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
    let mut url = Url::parse(&api.endpoint("sbom"))?;
    url.query_pairs_mut().append_pair("labels", labels);

    let file_bytes = fs::read(file_path).await?;

    let response = api
        .client()
        .post(url)
        .header("Content-Type", "application/octet-stream")
        .body(file_bytes)
        .send()
        .await?;

    Ok(response)
}

pub async fn upload_advisory(api: &LibClient, labels: &str, file_path: &Path) -> Result<Response, Box<dyn Error>> {
    let mut url = Url::parse(&api.endpoint("advisory"))?;
    url.query_pairs_mut().append_pair("labels", labels);

    let file_bytes = fs::read(file_path).await?;

    let response = api
        .client()
        .post(url)
        .header("Content-Type", "application/octet-stream")
        .body(file_bytes)
        .send()
        .await?;

    Ok(response)
}

pub async fn list_weaknesses(api: &LibClient) -> Result<Response, Box<dyn Error>> {
    let url = api.endpoint("weakness");
    let resp = api.client().get(&url).send().await?;
    Ok(resp)
}

pub async fn list_vulnerabilities(api: &LibClient) -> Result<Response, Box<dyn Error>> {
    let url = api.endpoint("vulnerability");
    let resp = api.client().get(&url).send().await?;
    Ok(resp)
}

pub async fn analyze(api: &LibClient, data: Value) -> Result<Response, Box<dyn Error>> {
    let url = api.endpoint("vulnerability/analyze");
    let resp = api.client().post(&url).json(&data).send().await?;
    Ok(resp)
}

pub async fn info(api: &LibClient) -> Result<Response, Box<dyn Error>> {
    let url = api.custom(".well-known/trustify");
    let resp = api.client().get(&url).send().await?;
    Ok(resp)
}
