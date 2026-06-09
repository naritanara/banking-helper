use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ASPSPGroup {
    name: String,
    logo: String, // TODO: uri type?
}