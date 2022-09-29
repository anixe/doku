use crate::prelude::*;

type Ty = [usize; 10];

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
