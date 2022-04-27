use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    foo: String,

    #[serde(skip_deserializing)]
    bar: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
