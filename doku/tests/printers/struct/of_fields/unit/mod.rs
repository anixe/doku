use crate::prelude::*;

#[derive(Default, Serialize, Document)]
struct Ty;

printer_test! {
    "output.ty.json" => to_json(Ty),
    "output.val.json" => to_json_val(Ty),
    "output.ty.toml" => to_toml(Ty),
    "output.val.toml" => to_toml_val(Ty),
}
