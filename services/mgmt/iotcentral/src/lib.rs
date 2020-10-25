#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-09-01")]
mod package_2018_09_01;
#[cfg(feature = "package-2018-09-01")]
pub use package_2018_09_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-07-01-privatepreview")]
mod package_2017_07_01_privatepreview;
#[cfg(feature = "package-2017-07-01-privatepreview")]
pub use package_2017_07_01_privatepreview::{models, operations, API_VERSION};
pub struct OperationConfig {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}
impl OperationConfig {
    pub fn new(bearer_access_token: &str) -> Self {
        Self {
            bearer_access_token: Some(bearer_access_token.to_owned()),
            ..Default::default()
        }
    }
}
impl Default for OperationConfig {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            bearer_access_token: None,
        }
    }
}
