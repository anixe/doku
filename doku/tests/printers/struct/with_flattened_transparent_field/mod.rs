use crate::prelude::*;

#[derive(Document)]
struct Ty {
    /// This is `f1`
    f1: String,

    /// This is `foo`
    #[doku(flatten)]
    foo: Foo,

    /// This is `f4`
    f4: String,
}

#[derive(Document)]
#[doku(transparent)]
struct Foo {
    /// This is `bar`
    bar: Bar,
}

#[derive(Document)]
struct Bar {
    /// This is `f2`
    f2: String,

    /// This is `f3`
    f3: String,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.without-comma.json" => to_json_without_comma(Ty),
    "output.without-key-quotes.json" => to_json_without_key_quotes(Ty),
}
