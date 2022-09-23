use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

printer_test! {
    "output.btreemap.json" => to_json(BTreeMap::<String, String>),
    "output.hashmap.json" => to_json(HashMap::<String, String>),
    "output.without_key_quotes.btreemap.json" => to_json_without_key_quotes(BTreeMap::<String, String>),
    "output.without_key_quotes.hashmap.json" => to_json_without_key_quotes(HashMap::<String, String>),
}
