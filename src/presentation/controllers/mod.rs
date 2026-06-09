use relm4::RelmApp;

use crate::{
    domain::DomainController,
    presentation::{icon_names, views::home::HomeView},
};

pub struct PresentationController {
    domain_controller: DomainController,
}

impl PresentationController {
    pub fn new(domain_controller: DomainController) -> Self {
        Self { domain_controller }
    }

    fn initialize_runtime(&self) {
        relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    }

    pub fn run(&self) {
        self.initialize_runtime();
        RelmApp::new("xyz.naritanara.banking-helper")
            .run::<HomeView>(self.domain_controller.get_application());
    }

    pub fn domain_controller(&self) -> &DomainController {
        &self.domain_controller
    }
}
