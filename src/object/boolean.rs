use super::{ObjectType, Objective};
use std::fmt::Display;

/// The Boolean object.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    /// Creates a new Boolean object.
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Objective for Boolean {
    fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
