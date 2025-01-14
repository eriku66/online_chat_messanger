use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserName {
    pub value: String,
    pub length: usize,
}

impl UserName {
    pub const MAX_LENGTH: usize = 255;

    pub fn new(value: String) -> Result<Self> {
        let trimmed_value = value.trim();

        if trimmed_value.as_bytes().len() > Self::MAX_LENGTH {
            return Err(anyhow!(
                "User name must be less than or equal to {} bytes",
                Self::MAX_LENGTH
            ));
        }

        Ok(Self {
            length: trimmed_value.len(),
            value: trimmed_value.to_string(),
        })
    }
}
