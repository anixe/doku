use doku::Document;

#[derive(Document)]
struct Person {
    #[doku(example)]
    name: String,
}

fn main() {
    println!("{}", doku::to_json::<Person>());
}