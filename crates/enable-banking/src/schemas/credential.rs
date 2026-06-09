use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credential {
    name: String,
    title: String,
    required: bool,
    description: Option<String>,
    template: Option<String>, // TODO: Perl REGEX type
}