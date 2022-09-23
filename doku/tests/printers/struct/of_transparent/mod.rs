use crate::prelude::*;

#[derive(Document)]
struct Ty {
    /// This is `foo`
    foo: Foo,

    /// This is `bar`
    bar: Bar,

    /// This is `zar`
    zar: Zar,
}

#[derive(Document)]
struct Foo {
    /// This is `f1`
    f1: String,

    /// This is `f2`
    f2: String,
}

#[derive(Document)]
#[doku(transparent)]
struct Bar(BarNested);

#[derive(Document)]
#[doku(transparent)]
struct BarNested {
    /// This is `f3`
    f3: String,
}

#[derive(Document)]
struct Zar {
    /// This is `f4`
    f4: String,

    /// This is `f5`
    f5: String,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.without_key_quotes.json" => to_json_without_key_quotes(Ty),
}
