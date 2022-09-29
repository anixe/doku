use crate::prelude::*;

#[derive(Document)]
struct Ty {
    f1: Vec<String>,

    #[doku(example = "f2-1")]
    f2: Vec<String>,

    #[doku(example = "f3-1", example = "f3-2")]
    f3: Vec<String>,

    #[doku(example = "f4-1", example = "f4-2")]
    #[doku(example = "f4-3")]
    f4: Option<Vec<String>>,

    #[doku(example = "f5-1")]
    f5: [String; 2],

    #[doku(example = "f6-1")]
    #[doku(example = "f6-2")]
    f6: [String; 2],
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
