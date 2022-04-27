use crate::prelude::*;

#[derive(Default, Serialize, Document)]
struct Ty {
    f1: F1,
    f2: F2,
    f3: F3,
}

#[derive(Serialize, Document)]
struct F1(String);

impl Default for F1 {
    fn default() -> Self {
        Self("I'm F1".to_string())
    }
}

#[derive(Serialize, Document)]
struct F2(String, usize);

impl Default for F2 {
    fn default() -> Self {
        Self("I'm F2".to_string(), 100)
    }
}

#[derive(Serialize, Document)]
struct F3(String, usize, f32);

impl Default for F3 {
    fn default() -> Self {
        Self("I'm F3".to_string(), 200, 300.5)
    }
}

printer_test! {
    "output.ty.json" => to_json(Ty),
    "output.val.json" => to_json_val(Ty),
}
