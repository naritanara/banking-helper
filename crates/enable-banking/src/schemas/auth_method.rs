use serde::{Deserialize, Serialize};

use crate::schemas::{authentication_approach::AuthenticationApproach, credential::Credential, psu_type::PSUType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthMethod {
    pub name: Option<String>,
    pub title: Option<String>,
    pub psu_type: PSUType,
    #[serde(default)]
    pub credentials: Vec<Credential>,
    pub approach: AuthenticationApproach,
    pub hidden_method: bool,
}