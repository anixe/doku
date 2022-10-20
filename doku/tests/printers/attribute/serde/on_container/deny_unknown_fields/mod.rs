use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(deny_unknown_fields)]
struct Ty;

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
