use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

printer_test! {
    "output.btreemap.json" => to_json(BTreeMap::<String, String>),
    "output.hashmap.json" => to_json(HashMap::<String, String>),
}
