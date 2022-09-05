use crate::prelude::*;

#[derive(Deserialize, Document)]
enum Ty {
    #[serde(alias = "Bar")]
    Foo,
}

printer_test! {
    "output.json" => to_json(Ty),
}
