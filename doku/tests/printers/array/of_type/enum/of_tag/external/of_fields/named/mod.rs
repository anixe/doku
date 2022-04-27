use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
enum Inner {
    /// This is `Foo`
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}

printer_test! {
    "output.json" => to_json(Ty),
}
