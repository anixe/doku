use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

#[derive(Document)]
enum Ty {
    /// This is `BTreeMap`
    BTreeMap(BTreeMap<String, String>),

    /// This is `HashMap`
    HashMap(HashMap<String, String>),
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.without-comma.json" => to_json_without_comma(Ty),
    "output.without-key-quotes.json" => to_json_without_key_quotes(Ty),
    "output.toml" => to_toml(Ty),
}
