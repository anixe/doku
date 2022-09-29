use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
struct Inner {
    /// Some comment
    foo: String,

    /// Some comment
    bar: usize,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
