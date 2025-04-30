use reqwest::{Client, Url};

#[derive(Clone)]
pub struct LibClient {
    base_url: String,
    client: Client,
}

impl LibClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    pub fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    pub fn base_host(&self) -> String {
        Url::parse(&self.base_url)
            .map(|u| {
                let scheme = u.scheme();
                let host = u.host_str().unwrap_or("localhost");
                match u.port() {
                    Some(port) => format!("{}://{}:{}", scheme, host, port),
                    None => format!("{}://{}", scheme, host),
                }
            })
            .unwrap_or_else(|_| "http://localhost:8080".to_string())
    }

    pub fn custom(&self, url: &str) -> String {
        let full_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!(
                "{}/{}",
                self.base_host().trim_end_matches('/'),
                url.trim_start_matches('/')
            )
        };

        println!("[api] custom url => {}", full_url);
        full_url
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
