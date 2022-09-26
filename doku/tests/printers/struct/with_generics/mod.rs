use crate::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Document)]
struct Generic<'a, K, V, const N: usize>
where
    K: Eq + Hash,
    V: Into<String>,
{
    map: HashMap<K, V>,
    s: &'a str,
    arr: [u32; N],
}

printer_test! {
    "output.numbers.json" => to_json(Generic<'static, u32, String, 3>),
    "output.strings.json" => to_json(Generic<'static, String, String, 4>),
}
