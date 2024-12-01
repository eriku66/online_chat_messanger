use crate::MAX_MESSAGE_SIZE_BYTES;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Message {
    length: usize,
    value: String,
}

impl Message {
    pub fn new(trimmed_value: String) -> Result<Self> {
        let trimmed_value = trimmed_value.trim();

        if trimmed_value.as_bytes().len() > MAX_MESSAGE_SIZE_BYTES {
            return Err(anyhow!(
                "Message must be less than or equal to {} bytes",
                MAX_MESSAGE_SIZE_BYTES
            ));
        }

        Ok(Self {
            length: trimmed_value.len(),
            value: trimmed_value.to_string(),
        })
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
