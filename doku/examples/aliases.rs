#![allow(dead_code)]

use doku::{Document, Fields, TypeKind};
use serde::Deserialize;

#[derive(Document, Deserialize)]
struct OurStruct {
    #[serde(alias = "foo")]
    foos: Vec<String>,

    #[serde(alias = "bar")]
    #[serde(alias = "baz")]
    bars: Vec<String>,
}

fn main() {
    if let doku::Type {
        kind:
            TypeKind::Struct {
                fields: Fields::Named { fields },
                ..
            },
        ..
    } = OurStruct::ty()
    {
        for (name, field) in fields {
            println!("Aliases for {}: {:?}", name, field.aliases);
        }
    } else {
        unreachable!();
    }
}
