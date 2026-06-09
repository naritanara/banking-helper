use serde::{Deserialize, Serialize};

use crate::schemas::{charge_bearer_code::ChargeBearerCode, payment_status::PaymentStatus, payment_type::PaymentType, priority_code::PriorityCode, psu_type::PSUType, reference_number_scheme::ReferenceNumberScheme, regulatory_reporting_code::RegulatoryReportingCode, remittance_information_line_info::RemittanceInformationLineInfo, scheme_name::SchemeName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsePaymentType {
    pub payment_type: PaymentType,
    pub max_transactions: Option<u64>,
    #[serde(default)]
    pub currencies: Vec<String>, // TODO: currency type?
    #[serde(default)]
    pub debtor_account_required: bool,
    #[serde(default)]
    pub debtor_account_schemas: Vec<SchemeName>,
    #[serde(default)]
    pub creditor_account_schemas: Vec<SchemeName>,
    #[serde(default)]
    pub priority_codes: Vec<PriorityCode>,
    #[serde(default)]
    pub charge_bearer_values: Vec<ChargeBearerCode>,
    #[serde(default)]
    pub creditor_country_required: bool,
    #[serde(default)]
    pub creditor_name_required: bool,
    #[serde(default)]
    pub creditor_postal_address_required: bool,
    #[serde(default)]
    pub remittance_information_required: bool,
    #[serde(default)]
    pub remittance_information_lines: Vec<RemittanceInformationLineInfo>,
    #[serde(default)]
    pub debtor_currency_required: bool,
    #[serde(default)]
    pub debtor_contact_email_required: bool,
    #[serde(default)]
    pub debtor_contact_phone_required: bool,
    #[serde(default)]
    pub creditor_agent_bic_fi_required: bool,
    #[serde(default)]
    pub creditor_agent_clearing_system_member_id_required: bool,
    #[serde(default)]
    pub allowed_auth_methods: Vec<String>,
    #[serde(default)]
    pub regulatory_reporting_codes: Vec<RegulatoryReportingCode>,
    #[serde(default)]
    pub regulatory_reporting_code_required: bool,
    #[serde(default)]
    pub reference_number_supported: bool,
    #[serde(default)]
    pub reference_number_schemas: Vec<ReferenceNumberScheme>,
    #[serde(default)]
    pub requested_execution_date_supported: bool,
    pub requested_execution_date_max_period: Option<u64>, // TODO: days type?
    #[serde(default)]
    pub remittance_reference_supported: bool,
    #[serde(default)]
    pub final_successful_statuses: Vec<PaymentStatus>,
    pub psu_type: PSUType,
}