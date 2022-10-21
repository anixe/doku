#![allow(dead_code)]

use doku::Document;

#[derive(Document)]
struct Response {
    event: Event,
}

#[derive(Document)]
enum Event {
    UserCreated { id: usize, name: String },
    UserDeleted { id: usize },
}

#[derive(Document)]
struct User {
    login: String,
}

fn main() {
    println!("```json");
    println!("{}", doku::to_json::<Response>());
    println!("```");
    println!();
    println!("```toml");
    println!("{}", doku::to_toml::<Response>());
    println!("```");
}
