use doku::Document;

#[derive(Document)]
struct Person {
    #[doku(example)]
    name: String,
}

fn main() {
    //
}
