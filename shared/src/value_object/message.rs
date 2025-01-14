use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Message {
    pub value: String,
    pub length: usize,
}

impl Message {
    pub const MAX_LENGTH: usize = 4096;

    pub fn new(trimmed_value: String) -> Result<Self> {
        let trimmed_value = trimmed_value.trim();

        if trimmed_value.as_bytes().len() > Self::MAX_LENGTH {
            return Err(anyhow!(
                "Message must be less than or equal to {} bytes",
                Self::MAX_LENGTH
            ));
        }

        Ok(Self {
            length: trimmed_value.len(),
            value: trimmed_value.to_string(),
        })
    }
}
