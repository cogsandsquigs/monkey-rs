pub mod boolean;
pub mod integer;
pub mod null;

use self::{boolean::Boolean, integer::Integer, null::Null};
use std::fmt::{Debug, Display};

/// An object in the Monkey programming language. This is the base trait for all
/// objects/values/types in the language. We use an enum wrapper instead of a
/// trait so that we are able to match on object values, as well as getting around
/// the trait-object limitation of not being able to use generic methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    /// The Integer object.
    Integer(Integer),

    /// The Boolean object.
    Boolean(Boolean),

    /// The Null object.
    Null(Null),
}

impl Objective for Object {
    fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null(_) => ObjectType::Null,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Integer(integer) => write!(f, "{}", integer),
            Object::Boolean(boolean) => write!(f, "{}", boolean),
            Object::Null(null) => write!(f, "{}", null),
        }
    }
}

/// The `Objective` trait represents an object in the Monkey programming language, which
/// requires some base functionality for any object going into the `Object` enum.
pub trait Objective: Debug + Display + Clone + PartialEq + Eq {
    /// Returns the type of the object.
    fn object_type(&self) -> ObjectType;

    /// Returns the string representation of the object.
    fn inspect(&self) -> String {
        self.to_string()
    }
}

/// The `ObjectType` enum represents the type of an object in the Monkey programming language.
/// This is used to match on the type of an object.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
    /// The Integer object.
    Integer,

    /// The Boolean object.
    Boolean,

    /// The Null object.
    Null,
}
