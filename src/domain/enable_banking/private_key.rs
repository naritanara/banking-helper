use std::path::Path;

use derive_more::Deref;
use jwt_simple::algorithms::RS256KeyPair;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

#[derive(Clone, Debug, Deref, Serialize, Deserialize)]
// #[serde(into = "Vec<u8>", try_from = "Vec<u8>")]
#[serde(into = "String", try_from = "String")]
pub struct PrivateKey(pub(super) RS256KeyPair);

impl PrivateKey {
    pub fn from_pem(pem: &str) -> Result<Self, PrivateKeyConstructorError> {
        Ok(Self(RS256KeyPair::from_pem(pem)?))
    }
}

#[derive(Debug, Error)]
pub enum PrivateKeyConstructorError {
    #[error("jwt-simple error")]
    JwtSimpleError(#[from] jwt_simple::Error),
}

impl PrivateKey {
    pub async fn load_from_file(path: impl AsRef<Path>) -> Result<Self, PrivateKeyLoadError> {
        let file = tokio::fs::read(path).await?;
        Ok(Self::from_pem(str::from_utf8(&file)?)?)
    }
}

#[derive(Debug, Error)]
pub enum PrivateKeyLoadError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("invalid utf-8")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    PrivateKeyConstructorError(#[from] PrivateKeyConstructorError),
}

impl TryFrom<Vec<u8>> for PrivateKey {
    type Error = PrivateKeyLoadError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::from_pem(str::from_utf8(&value)?)?)
    }
}

impl From<PrivateKey> for Vec<u8> {
    fn from(value: PrivateKey) -> Self {
        value
            .0
            .to_pem()
            .expect("serialization should succeed")
            .into_bytes()
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = PrivateKeyConstructorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_pem(&value)
    }
}

impl From<PrivateKey> for String {
    fn from(value: PrivateKey) -> Self {
        value.0.to_pem().unwrap()
    }
}
