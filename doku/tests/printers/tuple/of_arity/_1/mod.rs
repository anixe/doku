use crate::prelude::*;

type Ty = (String,);

printer_test! {
    "output.json" => to_json(Ty),
}
