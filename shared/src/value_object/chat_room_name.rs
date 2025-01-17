use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChatRoomName {
    pub value: String,
    pub length: usize,
}

impl ChatRoomName {
    pub const HEADER_LENGTH_BYTES: usize = 1;
    pub const MAX_LENGTH: usize = 255;
    pub const MAX_TOTAL_BYTES: usize = Self::HEADER_LENGTH_BYTES + Self::MAX_LENGTH;

    pub fn new(value: String) -> Result<Self> {
        let trimmed_value = value.trim();

        if trimmed_value.as_bytes().len() > Self::MAX_LENGTH {
            return Err(anyhow!(
                "Room name must be less than or equal to {} bytes",
                Self::MAX_LENGTH
            ));
        }

        Ok(Self {
            length: trimmed_value.len(),
            value: trimmed_value.to_string(),
        })
    }
}
