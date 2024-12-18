use enum_primitive_derive::Primitive;

#[derive(Debug, Clone, Copy, Primitive)]

pub enum OperationType {
    CreateChatRoom = 1,
    JoinChatRoom = 2,
}
