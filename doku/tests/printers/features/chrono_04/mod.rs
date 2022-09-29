use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Document)]
struct Ty {
    dt: DateTime<Utc>,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
