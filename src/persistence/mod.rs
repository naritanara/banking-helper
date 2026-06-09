use std::path::PathBuf;

use user_dirs::HomeDirError;

use crate::{
    domain::application::Application,
    persistence::settings::{Settings, SettingsLoadError, SettingsSaveError},
};

pub mod settings;

pub struct PersistenceController {
    settings_path: PathBuf,
}

impl PersistenceController {
    pub fn new(settings_path: impl Into<PathBuf>) -> Self {
        Self {
            settings_path: settings_path.into(),
        }
    }

    pub fn new_from_user_dirs() -> Result<Self, HomeDirError> {
        let settings_path = user_dirs::config_dir()?
            .join("banking-helper")
            .join("settings.toml");

        Ok(Self::new(settings_path))
    }

    pub fn load_settings(&self, application: &mut Application) -> Result<(), SettingsLoadError> {
        let settings = Settings::load(&self.settings_path)?;

        if let Some(api_credentials) = settings.api_credentials {
            application.set_api_credentials(api_credentials.clone());
        } else {
            application.remove_api_credentials();
        }

        Ok(())
    }

    pub fn save_settings(&self, application: &Application) -> Result<(), SettingsSaveError> {
        let settings = Settings {
            api_credentials: application.get_api_credentials().cloned(),
        };

        settings.save(&self.settings_path)?;

        Ok(())
    }
}
