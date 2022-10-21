#![allow(dead_code)]

use doku::Document;

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
    println!("```json");
    println!("{}", doku::to_json::<Type>());
    println!("```");
    println!();
    println!("```toml");
    println!("{}", doku::to_toml::<Type>());
    println!("```");
}
