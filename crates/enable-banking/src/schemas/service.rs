use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    #[serde(rename = "AIS")]
    AccountInformationService,
    #[serde(rename = "PIS")]
    PaymentInitiationService,
}