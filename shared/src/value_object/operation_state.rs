use enum_primitive_derive::Primitive;

#[derive(Debug, Clone, Copy, Primitive)]
pub enum OperationState {
    Request = 0,
    ReceiveResponse = 1,
    CompleteResponse = 2,
}
