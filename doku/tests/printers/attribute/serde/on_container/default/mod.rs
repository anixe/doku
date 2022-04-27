use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(default)]
struct Ty {
    f1: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
