use super::{UserName, UserToken};
use crate::ResponseStatus;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct OperationPayload {
    #[builder(setter(into, strip_option), default)]
    pub response_status: Option<ResponseStatus>,
    #[builder(setter(into, strip_option), default)]
    pub message: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub user_token: Option<UserToken>,
    #[builder(setter(into, strip_option), default)]
    pub user_name: Option<UserName>,
}

impl OperationPayload {
    pub const HEADER_LENGTH_BYTES: usize = 15;
    pub const MAX_LENGTH: usize = 2usize.pow(Self::HEADER_LENGTH_BYTES as u32) - 1;
    pub const MAX_TOTAL_BYTES: usize = Self::HEADER_LENGTH_BYTES + Self::MAX_LENGTH;
}
