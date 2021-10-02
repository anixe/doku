#![allow(dead_code)]

use doku::prelude::*;

#[derive(Document)]
struct Type {
    #[doku(example = "one")]
    foos: Vec<String>,

    #[doku(example = "one")]
    #[doku(example = "two")]
    bars: Vec<String>,

    #[doku(example = "one")]
    #[doku(example = "two")]
    zars: [String; 2],
}

fn main() {
    println!("{}", doku::to_json::<Type>());
}
