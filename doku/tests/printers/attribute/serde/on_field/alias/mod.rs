use crate::prelude::*;

#[derive(Deserialize, Document)]
struct Ty {
    #[serde(alias = "bar", alias = "zar")]
    foo: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
