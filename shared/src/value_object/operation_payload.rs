use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationPayload {
    success: bool,
    message: String,
}

impl OperationPayload {
    pub const HEADER_LENGTH_BYTES: usize = 29;
    pub const MAX_LENGTH: usize = 2usize.pow(Self::HEADER_LENGTH_BYTES as u32) - 1;
    pub const MAX_TOTAL_BYTES: usize = Self::HEADER_LENGTH_BYTES + Self::MAX_LENGTH;
}
