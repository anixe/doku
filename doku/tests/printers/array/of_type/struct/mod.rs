use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
struct Inner {
    /// Some comment
    foo: String,

    /// Some comment
    bar: usize,
}

printer_test! {
    "output.json" => to_json(Ty),
}
