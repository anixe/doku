use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    foo: String,

    #[serde(skip_serializing)]
    bar: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
