use crate::prelude::*;

#[derive(Deserialize, Document)]
enum Ty {
    #[serde(alias = "Bar", alias = "Zar")]
    Foo,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
