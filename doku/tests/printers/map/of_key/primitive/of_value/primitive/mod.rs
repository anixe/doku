use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

printer_test! {
    "output.btreemap.json" => to_json(BTreeMap::<String, String>),
    "output.hashmap.json" => to_json(HashMap::<String, String>),
    "output.without-comma.btreemap.json" => to_json_without_comma(BTreeMap::<String, String>),
    "output.without-comma.hashmap.json" => to_json_without_comma(HashMap::<String, String>),
    "output.without-key-quotes.btreemap.json" => to_json_without_key_quotes(BTreeMap::<String, String>),
    "output.without-key-quotes.hashmap.json" => to_json_without_key_quotes(HashMap::<String, String>),
}
