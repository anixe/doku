use crate::prelude::*;

#[derive(Deserialize, Document)]
struct Ty {
    #[doku(as = "usize")]
    foo: String,

    #[doku(as = "Vec<usize>")]
    bar: String,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
