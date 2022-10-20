use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(untagged)]
enum Ty {
    Foo { a: String },
    Bar { b: String },
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
