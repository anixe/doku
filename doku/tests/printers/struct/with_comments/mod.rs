use crate::prelude::*;

#[derive(Document)]
struct Ty {
    // This is `f1`
    f1: String,

    /// This is `f2`
    f2: String,

    /// This is `f3`
    /// ... and it's
    /// ... multiline!
    f3: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
