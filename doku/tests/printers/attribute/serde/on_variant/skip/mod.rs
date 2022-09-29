use crate::prelude::*;

#[derive(Serialize, Document)]
enum Ty {
    Foo,

    #[serde(skip)]
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
