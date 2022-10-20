use crate::prelude::*;

type Ty = Vec<usize>;

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
