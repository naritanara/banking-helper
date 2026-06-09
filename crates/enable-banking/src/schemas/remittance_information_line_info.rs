use std::ops::Bound;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemittanceInformationLineInfo {
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub pattern: Option<String>, // TODO: Regex type
}

impl RemittanceInformationLineInfo {
    pub fn length_range(&self) -> (Bound<u64>, Bound<u64>) {
        let lower_bound = self
            .min_length
            .map_or(Bound::Unbounded, Bound::Included);
        let upper_bound = self
            .max_length
            .map_or(Bound::Unbounded, Bound::Included);

        (lower_bound, upper_bound)
    }
}
