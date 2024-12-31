use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Primitive)]
pub enum OperationType {
    CreateChatRoom = 1,
    JoinChatRoom = 2,
}

impl OperationType {
    pub const HEADER_LENGTH_BYTES: usize = 1;

    pub fn from_u8(value: u8) -> Option<Self> {
        println!("value: {:?}", value);
        FromPrimitive::from_u8(value)
    }
}
