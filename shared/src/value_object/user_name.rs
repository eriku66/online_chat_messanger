use crate::MAX_USER_NAME_SIZE_BYTES;
use anyhow::{anyhow, Result};

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: String) -> Result<Self> {
        if value.as_bytes().len() > MAX_USER_NAME_SIZE_BYTES {
            return Err(anyhow!(
                "User name must be less than or equal to {} bytes",
                MAX_USER_NAME_SIZE_BYTES
            ));
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
