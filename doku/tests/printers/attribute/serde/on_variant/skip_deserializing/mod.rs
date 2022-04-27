use crate::prelude::*;

#[derive(Deserialize, Document)]
enum Ty {
    Foo,

    #[serde(skip_deserializing)]
    Bar,
}

printer_test! {
    "output.json" => to_json(Ty),
}
