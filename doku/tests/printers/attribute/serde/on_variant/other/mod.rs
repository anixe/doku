use crate::prelude::*;

#[derive(Serialize, Document)]
enum Ty {
    #[serde(other)]
    Foo,
}

printer_test! {
    "output.json" => to_json(Ty),
}
