use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
#[doku(tag = "t", content = "c")]
enum Inner {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
