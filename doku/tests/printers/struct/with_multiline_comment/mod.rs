use crate::prelude::*;

#[derive(Document)]
struct Ty {
    /// Comment for f1
    /// Another comment for f1
    /// Yet another comment for f1
    f1: Option<String>,
}

printer_test! {
    "output.json" => to_json(Ty),
}
