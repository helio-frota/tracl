use std::error::Error;

use reqwest::Response;
use serde_json::Value;

use crate::client::LibClient;

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
