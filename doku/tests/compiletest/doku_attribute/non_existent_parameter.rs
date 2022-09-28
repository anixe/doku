use doku::Document;

#[derive(Document)]
struct Person {
    #[doku(non_existent_parameter)]
    name: String,
}

fn main() {
    println!("{}", doku::to_json::<Person>());
}