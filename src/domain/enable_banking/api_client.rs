use std::time::Duration;

use enable_banking::{
    Client,
    client::{ClientCreationError, ClientQueryError},
    endpoints::application::GetApplication,
    schemas::service::Service,
};
use thiserror::Error;
use url::Host;

use crate::domain::enable_banking::api_credentials::ApiCredentials;

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new(
        api_credentials: &ApiCredentials,
        duration: &Duration,
    ) -> Result<Self, ApiClientConstructorError> {
        let application_id = (**api_credentials.application_id()).to_owned();
        let private_key = (*api_credentials.private_key()).to_owned();

        Ok(Self {
            client: Client::new(application_id, private_key.0, duration)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ApiClientConstructorError {
    #[error("enable banking client error")]
    EnableBankingClientError(#[from] ClientCreationError),
}

impl ApiClient {
    pub fn is_expired(&self) -> bool {
        self.client.is_expired()
    }
}

impl ApiClient {
    pub async fn test_client(&self) -> Result<(), TestClientError> {
        let application = self.client.query(&GetApplication).await?;

        let valid_redirect_url = application.redirect_urls.iter().any(|url| {
            url.host().is_some_and(|host| match host {
                Host::Domain(domain) => domain == "localhost",
                Host::Ipv4(ip) => ip.is_loopback(),
                Host::Ipv6(ip) => ip.is_loopback(),
            })
        });

        if !valid_redirect_url {
            return Err(TestClientError::NoValidRedirectUrl);
        }

        if !application
            .services
            .contains(&Service::AccountInformationService)
        {
            return Err(TestClientError::NoAccountInformationService);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TestClientError {
    #[error("api client library error")]
    ClientQueryError(#[from] ClientQueryError),
    #[error("the application does not have any redirect urls to a loopback address")]
    NoValidRedirectUrl,
    #[error("the application does not have access to the account information service")]
    NoAccountInformationService,
}
