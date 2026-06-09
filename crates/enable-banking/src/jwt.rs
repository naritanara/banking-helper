use std::time::{Duration, Instant, SystemTime};

use jwt::{Header, PKeyWithDigest, SignWithKey, Token, token::Signed};
use openssl::pkey::Private;
use reqwest::header::{HeaderName, HeaderValue};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct JwtBody {
    iss: String,
    aud: String,
    iat: u64,
    exp: u64,
}

impl JwtBody {
    pub fn new(duration: &Duration) -> Self {
        let now = SystemTime::now();
        let iat = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = iat + duration.as_secs();

        Self {
            iss: String::from("enablebanking.com"),
            aud: String::from("api.enablebanking.com"),
            iat,
            exp,
        }
    }
}

pub struct Jwt {
    token: Token<Header, JwtBody, Signed>,
    expiration: Instant,
}

impl Jwt {
    pub fn new(
        application_id: String,
        private_key: &PKeyWithDigest<Private>,
        duration: &Duration,
    ) -> Result<Self, JwtCreationError> {
        let header = Header {
            algorithm: jwt::AlgorithmType::Rs256,
            key_id: Some(application_id),
            ..Default::default()
        };
        
        let expiration = Instant::now() + *duration;
        let body = JwtBody::new(duration);
        let token = Token::new(header, body).sign_with_key(private_key)?;
        Ok(Self { token, expiration })
    }
}

#[derive(Debug, Error)]
pub enum JwtCreationError {
    #[error(transparent)]
    JwtError(#[from] jwt::Error),
}

impl Jwt {
    pub fn as_authorization_header(&self) -> (HeaderName, HeaderValue) {
        const HEADER_NAME: HeaderName = HeaderName::from_static("authorization");

        let raw_header_value = format!("Bearer {}", self.token.as_str());
        let mut header_value = HeaderValue::try_from(raw_header_value).unwrap();
        header_value.set_sensitive(true);

        (HEADER_NAME, header_value)
    }
    
    pub fn expiration(&self) -> &Instant {
        &self.expiration
    }
}
