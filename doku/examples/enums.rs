use doku::prelude::*;

#[derive(Doku)]
enum Event {
    UserCreated { id: usize, name: String },
    UserDeleted { id: usize },
}

#[derive(Doku)]
struct User {
    /// yass
    user: String,
}

fn main() {
    println!("Event:\n{}", doku::to_json::<Event>());
    println!();
    println!("Vec<Event>:\n{}", doku::to_json::<Vec<Event>>());
}
