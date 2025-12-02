use crate::endpoints::*;

use reqwest::{Client as Http, RequestBuilder};
use std::env;

pub struct LibClient {
    pub http: Http,
    pub base_url: String,
    pub token: Option<String>,
}

impl LibClient {
    /// Creates a new [`LibClient`].
    pub fn new(base_url: &str) -> Self {
        let token = env::var("TOKEN").ok();
        Self {
            http: Http::new(),
            base_url: base_url.to_string(),
            token,
        }
    }

    /// Adds an `Authorization` header to the request if a token is available.
    pub fn add_auth_header(&self, rb: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.token {
            rb.header("Authorization", format!("Bearer {token}"))
        } else {
            rb
        }
    }

    /// Returns a helper for interacting with organization endpoints.
    pub fn organization(&self) -> OrganizationApi<'_> {
        OrganizationApi::new(self)
    }
}

// #[derive(Clone)]
// pub struct LibClient {
//     base_url: String,
//     client: Client,
// }
//
// impl LibClient {
//     pub fn new(base_url: impl Into<String>) -> Self {
//         Self {
//             base_url: base_url.into(),
//             client: Client::new(),
//         }
//     }
//
//     pub fn endpoint(&self, path: &str) -> String {
//         format!(
//             "{}/{}",
//             self.base_url.trim_end_matches('/'),
//             path.trim_start_matches('/')
//         )
//     }
//
//     pub fn base_host(&self) -> String {
//         Url::parse(&self.base_url)
//             .map(|u| {
//                 let scheme = u.scheme();
//                 let host = u.host_str().unwrap_or("localhost");
//                 match u.port() {
//                     Some(port) => format!("{}://{}:{}", scheme, host, port),
//                     None => format!("{}://{}", scheme, host),
//                 }
//             })
//             .unwrap_or_else(|_| "http://localhost:8080".to_string())
//     }
//
//     pub fn custom_endpoint(&self, url: &str) -> String {
//         let full_url = if url.starts_with("http://") || url.starts_with("https://") {
//             url.to_string()
//         } else {
//             format!(
//                 "{}/{}",
//                 self.base_host().trim_end_matches('/'),
//                 url.trim_start_matches('/')
//             )
//         };
//
//         println!("[api] custom url => {}", full_url);
//         full_url
//     }
//
//     pub fn client(&self) -> &Client {
//         &self.client
//     }
// }
