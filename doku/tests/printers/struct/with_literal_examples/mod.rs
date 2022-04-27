use crate::prelude::*;

#[derive(Document)]
struct Ty {
    /// Comment for f1
    #[doku(literal_example = r#"[ "aaa", "bbb" ]"#)]
    f1: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
