use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxUser {
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "otp")]
    pub one_time_password: Option<String>,
}