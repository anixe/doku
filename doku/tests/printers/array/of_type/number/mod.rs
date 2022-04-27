use crate::prelude::*;

type Ty = Vec<usize>;

printer_test! {
    "output.json" => to_json(Ty),
}
