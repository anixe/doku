use crate::prelude::*;

type Ty = Option<Option<String>>;

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
