use anyhow::{anyhow, Context, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Primitive, PartialEq, Eq)]
pub enum OperationType {
    CreateChatRoom = 1,
    JoinChatRoom = 2,
}

impl OperationType {
    pub const HEADER_LENGTH_BYTES: usize = 1;

    pub fn try_from_u8(value: u8) -> Result<Self> {
        FromPrimitive::from_u8(value)
            .ok_or_else(|| anyhow!("Invalid operation type. value: {}", value))
            .context("Failed to parse operation type")
    }
}
