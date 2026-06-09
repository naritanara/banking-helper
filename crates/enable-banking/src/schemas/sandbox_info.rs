use serde::{Deserialize, Serialize};

use crate::schemas::sandbox_user::SandboxUser;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxInfo {
    #[serde(default)]
    pub users: Vec<SandboxUser>,
}