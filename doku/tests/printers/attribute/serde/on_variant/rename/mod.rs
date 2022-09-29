use crate::prelude::*;

#[derive(Serialize, Document)]
enum Ty {
    #[serde(rename = "Bar")]
    Foo,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
