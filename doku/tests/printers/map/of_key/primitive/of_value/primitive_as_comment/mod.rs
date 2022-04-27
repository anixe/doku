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
}
