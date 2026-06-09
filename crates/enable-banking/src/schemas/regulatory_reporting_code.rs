use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegulatoryReportingCode {
    pub value: String,
    pub description: String,
}