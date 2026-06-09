use banking_helper::{
    domain::DomainController, persistence::PersistenceController,
    presentation::controllers::PresentationController,
};

fn main() {
    let persistence_controller = PersistenceController::new_from_user_dirs().unwrap();
    let domain_controller = DomainController::new_empty(persistence_controller);
    let presentation_controller = PresentationController::new(domain_controller);

    let domain_controller = presentation_controller.domain_controller();
    if let Err(err) = domain_controller.load_settings() {
        eprintln!("Failed to load settings: {}", err);
    }

    presentation_controller.run();

    if let Err(err) = domain_controller.save_settings() {
        eprintln!("Failed to save settings: {}", err);
    }
}
