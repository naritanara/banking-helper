use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use jwt::PKeyWithDigest;
use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
};
use reqwest::{StatusCode, Url, header::HeaderMap};
use thiserror::Error;

use crate::{
    endpoints::Endpoint,
    jwt::{Jwt, JwtCreationError},
    schemas::error_response::ErrorResponse,
};

static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api.enablebanking.com").unwrap());

#[derive(Debug)]
pub struct Client {
    reqwest: reqwest::Client,
    expiration: Instant,
}

impl Client {
    pub fn new(
        application_id: String,
        private_key: PKey<Private>,
        duration: &Duration,
    ) -> Result<Self, ClientCreationError> {
        let private_key = PKeyWithDigest {
            digest: MessageDigest::sha256(),
            key: private_key,
        };

        let jwt = Jwt::new(application_id, &private_key, duration)?;

        let headers = HeaderMap::from_iter([jwt.as_authorization_header()]);

        let reqwest = reqwest::ClientBuilder::new()
            .https_only(true)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            reqwest,
            expiration: *jwt.expiration(),
        })
    }
}

#[derive(Debug, Error)]
pub enum ClientCreationError {
    #[error(transparent)]
    JwtCreationError(#[from] JwtCreationError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl Client {
    pub fn is_expired(&self) -> bool {
        self.expiration <= Instant::now()
    }
}

impl Client {
    pub async fn query<E: Endpoint>(&self, request: &E) -> Result<E::Response, ClientQueryError> {
        let base_url = BASE_URL.clone();
        let url = request.build_url(base_url);
        let response = self.reqwest.request(E::METHOD, url).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(response
                .json::<ErrorResponse>()
                .await
                .map(Into::into)
                .unwrap_or_else(ClientQueryError::ServerErrorInvalidResponse)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientQueryError {
    #[error("server anwsered with error")]
    ServerErrorResponse(#[from] ErrorResponse),
    #[error("server anwsered with error, but the response could not be parsed")]
    ServerErrorInvalidResponse(#[source] reqwest::Error),
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
}
