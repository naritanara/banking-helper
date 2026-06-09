use std::sync::Arc;

use crate::{
    domain::application::{Application, SharedApplication},
    persistence::{
        PersistenceController,
        settings::{SettingsLoadError, SettingsSaveError},
    },
};

pub mod application;
pub mod enable_banking;

pub struct DomainController {
    application: SharedApplication,
    persistence_controller: PersistenceController,
}

impl DomainController {
    pub fn new(application: Application, persistence_controller: PersistenceController) -> Self {
        Self {
            application: Arc::new(application.into()),
            persistence_controller,
        }
    }

    pub fn new_empty(persistence_controller: PersistenceController) -> Self {
        Self::new(Application::default(), persistence_controller)
    }

    pub fn new_from_persistence(
        persistence_controller: PersistenceController,
    ) -> Result<Self, SettingsLoadError> {
        let controller = Self::new_empty(persistence_controller);
        controller.load_settings()?;
        Ok(controller)
    }

    pub fn get_application(&self) -> SharedApplication {
        self.application.clone()
    }

    pub fn load_settings(&self) -> Result<(), SettingsLoadError> {
        self.persistence_controller
            .load_settings(&mut self.get_application().blocking_write())
    }

    pub fn save_settings(&self) -> Result<(), SettingsSaveError> {
        self.persistence_controller
            .save_settings(&self.get_application().blocking_read())
    }
}
