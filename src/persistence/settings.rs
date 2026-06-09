use std::{io, path::Path};

use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

use crate::domain::enable_banking::api_credentials::ApiCredentials;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub(super) api_credentials: Option<ApiCredentials>,
}

impl Settings {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, SettingsLoadError> {
        let file_contents = fs::read(path)?;
        Ok(toml::from_slice(&file_contents)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), SettingsSaveError> {
        let file_contents = toml::to_string_pretty(self)?;

        if let Some(parent_dir) = path.as_ref().parent() {
            fs::create_dir_all(parent_dir)?;
        }

        fs::write(path, file_contents)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum SettingsLoadError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("deserialization error")]
    DeserializationError(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum SettingsSaveError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("serialization error")]
    SerializationError(#[from] toml::ser::Error),
}
