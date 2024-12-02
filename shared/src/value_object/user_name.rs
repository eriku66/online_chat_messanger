use crate::MAX_USER_NAME_SIZE_BYTES;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct UserName {
    length: usize,
    value: String,
}

impl UserName {
    pub fn new(value: String) -> Result<Self> {
        let trimmed_value = value.trim();

        if trimmed_value.as_bytes().len() > MAX_USER_NAME_SIZE_BYTES {
            return Err(anyhow!(
                "User name must be less than or equal to {} bytes",
                MAX_USER_NAME_SIZE_BYTES
            ));
        }

        Ok(Self {
            length: trimmed_value.len(),
            value: trimmed_value.to_string(),
        })
    }

    pub fn length(&self) -> usize {
        self.value.len()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
