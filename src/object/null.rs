use super::{ObjectType, Objective};
use std::fmt::Display;

/// The Null object.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Null;

impl Null {
    /// Creates a new Null object.
    pub fn new() -> Self {
        Self
    }
}

impl Objective for Null {
    fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "null")
    }
}
