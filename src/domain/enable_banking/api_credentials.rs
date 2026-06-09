use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::enable_banking::{
    api_client::{ApiClient, ApiClientConstructorError, TestClientError}, application_id::ApplicationId, private_key::PrivateKey,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiCredentials {
    application_id: ApplicationId,
    private_key: PrivateKey,
}

impl ApiCredentials {
    pub fn new(application_id: ApplicationId, private_key: PrivateKey) -> Self {
        Self {
            application_id,
            private_key,
        }
    }

    pub fn application_id(&self) -> &ApplicationId {
        &self.application_id
    }

    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }
}

impl ApiCredentials {
    pub async fn test_credentials(&self) -> Result<(), TestCredentialsError> {
        let client = ApiClient::new(self, &Duration::from_secs(30))?;
        client.test_client().await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TestCredentialsError {
    #[error("error constructing API client")]
    ApiClientConstructorError(#[from] ApiClientConstructorError),
    #[error("error executing test request")]
    TestClientError(#[from] TestClientError),
}