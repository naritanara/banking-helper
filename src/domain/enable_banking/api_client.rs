use std::str::FromStr;

use enable_banking::raw_api::{
    ApiOpError, GetApplicationApplicationGetApiError, HttpClient, Service,
};
use jwt_simple::{prelude::*, reexports::coarsetime::Instant};
use thiserror::Error;
use url::Host;

use crate::domain::enable_banking::api_credentials::ApiCredentials;

pub struct ApiClient {
    client: HttpClient,
    jwt_expiration: Instant,
}

impl ApiClient {
    pub fn new(
        api_credentials: &ApiCredentials,
        duration: Duration,
    ) -> Result<Self, ApiClientConstructorError> {
        let (api_key, jwt_expiration) = api_credentials.issue_api_key(duration)?;

        Ok(Self {
            client: HttpClient::new().with_api_key(api_key),
            jwt_expiration,
        })
    }
}

#[derive(Debug, Error)]
pub enum ApiClientConstructorError {
    #[error("jwt generation error")]
    JwtSimpleError(#[from] jwt_simple::Error),
}

impl ApiClient {
    pub fn is_expired(&self) -> bool {
        self.jwt_expiration < Instant::now()
    }
}

impl ApiClient {
    pub async fn test_client(&self) -> Result<(), TestClientError> {
        let application = self.client.get_application_application_get().await?;

        let valid_redirect_url = application.redirect_urls.iter().any(|url| {
            let url = url::Url::from_str(url).unwrap();
            url.host().is_some_and(|host| match host {
                Host::Domain(domain) => domain == "localhost",
                Host::Ipv4(ip) => ip.is_loopback(),
                Host::Ipv6(ip) => ip.is_loopback(),
            })
        });

        if !valid_redirect_url {
            return Err(TestClientError::NoValidRedirectUrl);
        }

        if !application.services.contains(&Service::Ais) {
            return Err(TestClientError::NoAccountInformationService);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TestClientError {
    #[error("api client library error")]
    ClientQueryError(#[from] Box<ApiOpError<GetApplicationApplicationGetApiError>>),
    #[error("the application does not have any redirect urls to a loopback address")]
    NoValidRedirectUrl,
    #[error("the application does not have access to the account information service")]
    NoAccountInformationService,
}

impl From<ApiOpError<GetApplicationApplicationGetApiError>> for TestClientError {
    fn from(value: ApiOpError<GetApplicationApplicationGetApiError>) -> Self {
        Box::new(value).into()
    }
}
