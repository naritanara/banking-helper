use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemeName {
    #[serde(rename = "ARNU")]
    AlienRegistrationNumber,
    #[serde(rename = "BANK")]
    BankPartyIdentification,
    #[serde(rename = "BBAN")]
    BasicBankAccountNumber,
    #[serde(rename = "BGNR")]
    BankGiroAccountNumber,
    #[serde(rename = "CCPT")]
    PassportNumber,
    #[serde(rename = "CHID")]
    ClearingIdentificationNumber,
    #[serde(rename = "COID")]
    CountryIdentificationNumber,
    #[serde(rename = "CPAN")]
    CardPAN,
    #[serde(rename = "CUSI")]
    CustomerIdentificationNumberIndividual,
    #[serde(rename = "CUST")]
    CorporateCustomerNumber,
    #[serde(rename = "DRLC")]
    DriversLicenseNumber,
    #[serde(rename = "DUNS")]
    DataUniversalNumberingSystem,
    #[serde(rename = "EMPL")]
    EmployerIdentificationNumber,
    #[serde(rename = "GS1G")]
    GS1GLNIdentifier,
    IBAN,
    #[serde(rename = "MIBN")]
    MaskedIBAN,
    #[serde(rename = "NIDN")]
    NationalIdentityNumber,
    #[serde(rename = "OAUT")]
    OAuth2,
    #[serde(rename = "OTHC")]
    OtherCorporate,
    #[serde(rename = "OTHI")]
    OtherIndividual,
    #[serde(rename = "PGNR")]
    PlusGiroAccountNumber,
    #[serde(rename = "SOSE")]
    SocialSecurityNumber,
    #[serde(rename = "SREN")]
    SIREN,
    #[serde(rename = "SRET")]
    SIRET,
    #[serde(rename = "TXID")]
    TaxIdentificationNumber,
}