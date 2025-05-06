use std::env;

#[derive(Debug, Clone)]
pub struct LibConfig {
    pub api_base_url: String,
}

impl Default for LibConfig {
    fn default() -> Self {
        Self {
            api_base_url: "http://localhost:8080/api/v2".to_string(),
        }
    }
}

impl LibConfig {
    pub fn from_env() -> Self {
        let api_base_url =
            env::var("TRUSTIFY_URL").unwrap_or_else(|_| LibConfig::default().api_base_url);
        Self { api_base_url }
    }
}
