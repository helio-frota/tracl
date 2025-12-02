use crate::client::LibClient;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrganizationList {
    pub items: Vec<Organization>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub cpe_key: Option<String>,
    pub website: Option<String>,
}

pub struct OrganizationApi<'a> {
    client: &'a LibClient,
}

impl<'a> OrganizationApi<'a> {
    pub fn new(client: &'a LibClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: u32) -> Result<Organization, Error> {
        let url = format!("{}/api/v2/organization/{}", self.client.base_url, id);
        let rb = self.client.http.get(url);
        let rb = self.client.add_auth_header(rb);
        let resp = rb.send().await?.json().await?;
        Ok(resp)
    }

    pub async fn get_all(&self) -> Result<OrganizationList, Error> {
        let url = format!("{}/api/v2/organization", self.client.base_url);
        let rb = self.client.http.get(url);
        let rb = self.client.add_auth_header(rb);
        let resp = rb.send().await?.json().await?;
        Ok(resp)
    }
}
