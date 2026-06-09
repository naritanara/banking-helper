use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::schemas::error_code::ErrorCode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("server returned error: {message}")]
pub struct ErrorResponse {
    pub message: String,
    pub code: Option<u64>,
    pub error: Option<ErrorCode>,
    #[serde(default)]
    pub detail: serde_json::Value,
}