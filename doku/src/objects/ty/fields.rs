use crate::*;

#[derive(Clone, Debug)]
pub enum Fields {
    /// E.g.: `struct Foo { a: usize, b: String }`
    Named { fields: Vec<(&'static str, ty::Field)> },

    /// E.g.: `struct Foo(usize, String);`
    Unnamed { fields: Vec<ty::Field> },

    /// E.g.: `struct Foo;`
    Unit,
}
