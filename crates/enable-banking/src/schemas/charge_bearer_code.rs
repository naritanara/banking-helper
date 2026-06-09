use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargeBearerCode {
    #[serde(rename = "CRED")]
    Creditor,
    #[serde(rename = "DEBT")]
    Debtor,
    #[serde(rename = "SHAR")]
    Shared,
    #[serde(rename = "SLEV")]
    ServiceLevel,
}