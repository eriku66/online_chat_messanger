use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Primitive)]
pub enum OperationType {
    CreateChatRoom = 1,
    JoinChatRoom = 2,
}

impl OperationType {
    pub fn from_u8(value: u8) -> Option<Self> {
        FromPrimitive::from_u8(value)
    }
}
