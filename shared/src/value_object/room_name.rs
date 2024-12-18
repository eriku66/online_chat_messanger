use crate::MAX_ROOM_NAME_SIZE_BYTES;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct RoomName {
    length: usize,
    value: String,
}

impl RoomName {
    pub fn new(value: String) -> Result<Self> {
        let trimmed_value = value.trim();

        if trimmed_value.as_bytes().len() > MAX_ROOM_NAME_SIZE_BYTES {
            return Err(anyhow!(
                "Room name must be less than or equal to {} bytes",
                MAX_ROOM_NAME_SIZE_BYTES
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
