use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserToken {
    pub value: String,
    pub length: usize,
}

impl Default for UserToken {
    fn default() -> Self {
        let value = Uuid::new_v4().to_string();
        let length = value.len();

        Self { value, length }
    }
}

impl UserToken {
    pub const HEADER_LENGTH_BYTES: usize = 1;
    pub const LENGTH: usize = 36;
    pub const TOTAL_BYTES: usize = Self::HEADER_LENGTH_BYTES + Self::LENGTH;

    pub fn new(value: String) -> Self {
        let length = value.len();

        Self { value, length }
    }
}
