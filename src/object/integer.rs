use super::{ObjectType, Objective};
use std::fmt::Display;

/// The Integer object.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    /// Creates a new Integer object.
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Objective for Integer {
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
