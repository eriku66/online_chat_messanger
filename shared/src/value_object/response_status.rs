use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Primitive, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    Ok = 200,
    BadRequest = 400,
}

impl ResponseStatus {
    pub fn from_u16(value: u16) -> Option<Self> {
        FromPrimitive::from_u16(value)
    }
}
