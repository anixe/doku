use crate::prelude::*;

#[derive(Document)]
struct Ty {
    /// This is `foo`
    foo: Foo,

    /// This is `bar`
    #[doku(flatten)]
    bar: Bar,

    /// This is `zar`
    zar: Zar,
}

#[derive(Document)]
struct Foo {
    f1: String,
    f2: String,
}

#[derive(Document)]
struct Bar {
    f3: String,
    f4: String,
    f5: BarNested,
}

#[derive(Document)]
struct BarNested {
    f6: String,
}

#[derive(Document)]
struct Zar {
    f7: String,
    f8: String,
}

printer_test! {
    "output.json" => to_json(Ty),
}
