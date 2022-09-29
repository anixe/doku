use crate::prelude::*;

#[derive(Document)]
struct Ty {
    f1: F1,
    f2: F2,
    f3: F3,
}

#[derive(Document)]
struct F1 {
    f11: F2,
    f12: String,
}

#[derive(Document)]
struct F2 {
    f21: F3,
}

#[derive(Document)]
struct F3 {
    f31: u32,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
