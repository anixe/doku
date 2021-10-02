#![allow(dead_code)]

use doku::prelude::*;

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
    println!("{}", doku::to_json::<Response>());
}
