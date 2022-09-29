use crate::prelude::*;

type Ty = (String, usize, Vec<String>);

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
