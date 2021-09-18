// run: to_json()

use std::collections::HashMap;

#[derive(Document)]
pub enum Ty {
    /// This is `Foo`
    Foo(HashMap<String, String>),
}
