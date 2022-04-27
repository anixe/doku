use crate::prelude::*;

type Ty = Option<Option<String>>;

printer_test! {
    "output.json" => to_json(Ty),
}
