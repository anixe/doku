use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    foo: String,

    #[serde(skip_serializing_if = "fun")]
    bar: String,
}

fn fun(_: &str) -> bool {
    unimplemented!()
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
