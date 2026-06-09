use serde::{Deserialize, Serialize};

use crate::schemas::{aspsp_group::ASPSPGroup, auth_method::AuthMethod, psu_type::PSUType, response_payment_type::ResponsePaymentType, sandbox_info::SandboxInfo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ASPSPData {
    pub name: String,
    pub country: String,
    pub logo: String,
    pub psu_types: Vec<PSUType>,
    pub auth_methods: Vec<AuthMethod>,
    pub maximum_consent_validity: u64, // TODO: seconds type
    pub sandbox: Option<SandboxInfo>,
    pub beta: bool,
    pub bic: Option<String>, // TODO: bic type?
    #[serde(default)]
    pub required_psu_headers: Vec<String>,
    #[serde(default)]
    pub payments: Vec<ResponsePaymentType>,
    pub group: Option<ASPSPGroup>,
}