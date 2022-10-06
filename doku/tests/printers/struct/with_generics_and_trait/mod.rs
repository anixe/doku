use std::fmt::Display;

use crate::prelude::*;

trait Trait {}

#[derive(Document)]
struct Generic<D: Display> {
    inner: D,
}

printer_test! {
    "output.json" => to_json(Generic<String>),
}
