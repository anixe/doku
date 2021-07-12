use crate::*;

#[derive(Clone, Debug)]
pub enum Fields {
    /// E.g.: `struct Foo { a: usize, b: String }`
    Named { fields: Vec<(&'static str, Field)> },

    /// E.g.: `struct Foo(usize, String);`
    Unnamed { fields: Vec<Field> },

    /// E.g.: `struct Foo;`
    Unit,
}
