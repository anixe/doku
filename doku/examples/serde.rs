use doku::Document;
use serde::Serialize;

#[derive(Serialize, Document)]
struct Response {
    #[serde(flatten)]
    pagination: PaginationWrapper,

    #[serde(rename = "items")]
    users: Vec<User>,
}

#[derive(Serialize, Document)]
#[serde(transparent)]
struct PaginationWrapper(Pagination);

#[derive(Serialize, Document)]
struct Pagination {
    current_page: usize,
    last_page: usize,
}

#[derive(Serialize, Document)]
struct User {
    #[doku(example = "alan.turing")]
    login: String,

    #[doku(example = "lofi hip hop radio")]
    favorite_radio: String,
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
