use crate::prelude::*;

type Ty = (String, usize, Vec<String>);

printer_test! {
    "output.json" => to_json(Ty),
}
