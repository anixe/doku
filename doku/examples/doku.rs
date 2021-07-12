use doku::prelude::*;

type People = Vec<Person>;

#[derive(Doku)]
struct Person {
    /// Person's first name
    #[doku(example = "Janet")]
    name: String,

    /// Person's last name
    #[doku(example = "NotARobot")]
    surname: Option<String>,

    /// Person's favourite color
    favorite_color: Color,
}

#[derive(Doku)]
enum Color {
    #[doku(rename = "red-uwu")]
    Red,

    #[doku(rename = "green-uwu")]
    Green,

    #[doku(rename = "bluwu")]
    Blue,
}

fn main() {
    println!("{}", doku::to_json::<People>());
}
