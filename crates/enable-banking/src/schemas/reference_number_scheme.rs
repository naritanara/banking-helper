use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ReferenceNumberScheme {
    #[serde(rename = "BERF")]
    BelgianReferenceNumber,
    #[serde(rename = "FIRF")]
    FinnishReferenceNumber,
    #[serde(rename = "INTL")]
    InternationalReferenceNumber,
    #[serde(rename = "NORF")]
    NorwegianKID,
    #[serde(rename = "SDDM")]
    SEPADirectDebitMandateID,
    #[serde(rename = "SEBG")]
    BankgiroOCR,
}