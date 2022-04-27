use crate::prelude::*;

type Ty = Option<String>;

printer_test! {
    "output.json" => to_json(Ty),
}
