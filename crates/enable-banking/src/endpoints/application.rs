use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};

use crate::{
    endpoints::Endpoint,
    schemas::{environment::Environment, service::Service},
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetApplication;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetApplicationResponse {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "kid")]
    pub key_id: String,
    pub environment: Environment,
    pub redirect_urls: Vec<Url>,
    pub active: bool,
    pub countries: Vec<String>,
    pub services: Vec<Service>,
}

impl Endpoint for GetApplication {
    const METHOD: Method = Method::GET;

    type Response = GetApplicationResponse;

    fn build_url(&self, mut base_url: Url) -> Url {
        base_url.set_path("application");

        base_url
    }
}
