use std::collections::HashMap;

#[derive(Doku)]
pub enum Ty {
    /// This is `Foo`
    Foo(HashMap<String, String>),
}
