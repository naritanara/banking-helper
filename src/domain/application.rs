use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::enable_banking::api_credentials::ApiCredentials;

#[derive(Default)]
pub struct Application {
    api_credentials: Option<ApiCredentials>,
}

impl Application {
    pub fn get_api_credentials(&self) -> Option<&ApiCredentials> {
        self.api_credentials.as_ref()
    }

    pub fn set_api_credentials(&mut self, api_credentials: ApiCredentials) -> &ApiCredentials {
        self.api_credentials.insert(api_credentials)
    }
    
    pub fn remove_api_credentials(&mut self)  {
        self.api_credentials = None;
    }
}

pub type SharedApplication = Arc<RwLock<Application>>;