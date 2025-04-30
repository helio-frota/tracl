use std::error::Error;

use reqwest::Response;

use crate::client::LibClient;

pub async fn info(api: &LibClient) -> Result<Response, Box<dyn Error>> {
    let url = api.custom(".well-known/trustify");
    let resp = api.client().get(&url).send().await?;
    Ok(resp)
}
