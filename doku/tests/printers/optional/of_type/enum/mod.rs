use crate::prelude::*;

type Ty = Option<Enum>;

#[derive(Document)]
enum Enum {
    Foo,
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
