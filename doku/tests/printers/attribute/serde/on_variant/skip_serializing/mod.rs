use crate::prelude::*;

#[derive(Serialize, Document)]
enum Ty {
    Foo,

    #[serde(skip_serializing)]
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
