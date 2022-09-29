use crate::prelude::*;

type Ty = Vec<Inner>;

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
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
