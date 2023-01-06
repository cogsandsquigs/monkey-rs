use super::{ObjectType, Objective};
use std::fmt::Display;

/// The Null object.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Null;

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
