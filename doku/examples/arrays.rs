#![allow(dead_code)]

use doku::prelude::*;

#[derive(Document)]
struct Type {
    #[doku(example = "one")]
    foos: Vec<String>,

    #[doku(example = r#"["one", "two", /* ... */]"#)]
    bars: Vec<String>,

    #[doku(example = "[\n\t\"one\",\n\t\"two\",\n\t/* ... */\n]")]
    zars: Vec<String>,
}

fn main() {
    println!("{}", doku::to_json::<Type>());
}
