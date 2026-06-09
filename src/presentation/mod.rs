pub mod views;
pub mod controllers;

mod icon_names {
    pub use shipped::*; // Include all shipped icons by default
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
