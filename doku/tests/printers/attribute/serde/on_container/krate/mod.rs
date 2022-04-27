use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(crate = "::serde")]
struct Ty {
    foo: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
