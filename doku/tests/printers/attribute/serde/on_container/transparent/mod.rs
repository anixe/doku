use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    foo: Foo,
}

#[derive(Serialize, Document)]
#[serde(transparent)]
struct Foo {
    bar: Bar,
}

#[derive(Serialize, Document)]
struct Bar {
    f1: String,
    f2: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
