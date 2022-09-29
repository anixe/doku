use crate::prelude::*;
use std::time::Duration;

#[derive(Document)]
struct Ty {
    tt: Duration,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
