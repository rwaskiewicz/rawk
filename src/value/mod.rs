use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueType {
    Number,
}

#[derive(Copy, Clone)]
pub union As {
    pub number: f32,
}

#[derive(Clone, Copy)]
pub struct Value {
    pub value_type: ValueType,
    pub type_as: As,
}

impl Value {
    pub fn new(value_type: ValueType, type_as: As) -> Value {
        Value {
            value_type,
            type_as,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.value_type {
            ValueType::Number => unsafe { write!(f, "{}", self.type_as.number) },
        }
    }
}
