use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    BulkDomestic,
    BulkDomesticSeGiro,
    BulkSEPA,
    Crossborder,
    Domestic,
    DomesticSeGiro,
    InstSEPA,
    Internal,
    SEPA,
}