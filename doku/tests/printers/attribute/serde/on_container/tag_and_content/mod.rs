use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(tag = "t", content = "c")]
enum Ty {
    Foo { a: String },
    Bar { a: String },
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
