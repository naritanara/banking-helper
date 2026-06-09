use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum PriorityCode {
    #[serde(rename = "EXPR")]
    Express,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Normal,
}