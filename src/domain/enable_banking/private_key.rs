use std::path::Path;

use derive_more::Deref;
use openssl::pkey::{PKey, Private};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

#[derive(Clone, Debug, Deref, Serialize, Deserialize)]
// #[serde(into = "Vec<u8>", try_from = "Vec<u8>")]
#[serde(into = "String", try_from = "String")]
pub struct PrivateKey(pub(super) PKey<Private>);

impl PrivateKey {
    pub fn from_pem(pem: &[u8]) -> Result<Self, PrivateKeyConstructorError> {
        Ok(Self(PKey::private_key_from_pem(pem)?))
    }
}

#[derive(Debug, Error)]
pub enum PrivateKeyConstructorError {
    #[error("openssl error")]
    OpenSSLError(#[from] openssl::error::ErrorStack),
}

impl PrivateKey {
    pub async fn load_from_file(path: impl AsRef<Path>) -> Result<Self, PrivateKeyLoadError> {
        let file = tokio::fs::read(path).await?;
        Ok(Self::from_pem(&file)?)
    }
}

#[derive(Debug, Error)]
pub enum PrivateKeyLoadError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    PrivateKeyConstructorError(#[from] PrivateKeyConstructorError),
}

impl TryFrom<Vec<u8>> for PrivateKey {
    type Error = PrivateKeyConstructorError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_pem(&value)
    }
}

impl From<PrivateKey> for Vec<u8> {
    fn from(value: PrivateKey) -> Self {
        value.0.private_key_to_pem_pkcs8().expect("serialization should succeed")
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = PrivateKeyConstructorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_pem(value.as_bytes())
    }
}

impl From<PrivateKey> for String {
    fn from(value: PrivateKey) -> Self {
        let bytes = value.0.private_key_to_pem_pkcs8().unwrap();
        String::from_utf8(bytes).unwrap()
    }
}