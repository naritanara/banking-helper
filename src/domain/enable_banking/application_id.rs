use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApplicationId(String);

impl ApplicationId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Deref for ApplicationId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
