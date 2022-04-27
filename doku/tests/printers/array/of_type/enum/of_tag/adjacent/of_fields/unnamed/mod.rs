use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
#[doku(tag = "t", content = "c")]
enum Inner {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}

printer_test! {
    "output.json" => to_json(Ty),
}
