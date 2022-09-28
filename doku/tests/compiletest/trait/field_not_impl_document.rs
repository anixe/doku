use doku::Document;

#[derive(Document)]
struct Foo {
  bar: Bar,
}

struct Bar;

fn main() {
    println!("{}", doku::to_json::<Foo>());
}