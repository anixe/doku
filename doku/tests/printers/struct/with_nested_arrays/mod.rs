use crate::prelude::*;

#[derive(Document)]
struct Ty {
    foos: Vec<Foo>,
}

#[derive(Document)]
struct Foo {
    bars: Vec<Bar>,
}

#[derive(Document)]
struct Bar {
    val: String,
    zars: Vec<Zar>,
}

#[derive(Document)]
struct Zar {
    value: String,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
