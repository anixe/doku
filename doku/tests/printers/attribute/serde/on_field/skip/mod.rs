use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    foo: String,

    #[serde(skip)]
    bar: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
