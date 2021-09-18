#![allow(dead_code)]

use doku::prelude::*;

#[derive(Document)]
enum Event {
    UserCreated { id: usize, name: String },
    UserDeleted { id: usize },
}

#[derive(Document)]
struct User {
    /// yass
    user: String,
}

fn main() {
    println!("Event:\n{}", doku::to_json::<Event>());
    println!();
    println!("Vec<Event>:\n{}", doku::to_json::<Vec<Event>>());
}
