use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    #[serde(default)]
    foo: bool,
}

printer_test! {
    "output.json" => to_json(Ty),
}
