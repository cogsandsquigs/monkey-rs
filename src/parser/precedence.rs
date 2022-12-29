#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,   // ==
    Ordering, // > or <. Note that in the original implementation, this is called `LessGreater`.
    Sum,      // +
    Product,  // *
    Prefix,   // -X or !X
    Call,     // myFunction(X)
}
