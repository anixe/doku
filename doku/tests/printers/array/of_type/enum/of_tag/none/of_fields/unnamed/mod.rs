use crate::prelude::*;

#[derive(Document)]
struct Ty {
    inner: Vec<Inner>,
}

#[derive(Document)]
struct Inner {
    /// Some comment
    foo: String,

    /// Another comment
    bar: String,

    /// Payload
    #[doku(tag = "tag")]
    payload: Enum,
}

#[derive(Document)]
#[doku(untagged)]
enum Enum {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
