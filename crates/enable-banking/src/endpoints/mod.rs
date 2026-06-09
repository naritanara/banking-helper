use reqwest::{Method, Url};
use serde::{Serialize, de::DeserializeOwned};

pub mod application;
pub mod aspsps;

pub trait Endpoint: Serialize {
    const METHOD: Method;
    
    type Response: DeserializeOwned;
    
    fn build_url(&self, base_url: Url) -> Url;
}
