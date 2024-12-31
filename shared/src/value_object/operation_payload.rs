use serde::{Deserialize, Serialize};

use crate::ResponseStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationPayload {
    status: ResponseStatus,
    message: Option<String>,
}

impl OperationPayload {
    pub const HEADER_LENGTH_BYTES: usize = 15;
    pub const MAX_LENGTH: usize = 2usize.pow(Self::HEADER_LENGTH_BYTES as u32) - 1;
    pub const MAX_TOTAL_BYTES: usize = Self::HEADER_LENGTH_BYTES + Self::MAX_LENGTH;

    pub fn new(status: ResponseStatus, message: Option<String>) -> Self {
        Self { status, message }
    }
}
