use crate::prelude::*;

#[derive(Document)]
#[doku(untagged)]
enum Ty {
    Foo(Foo),
    Bar(Bar),
}

#[derive(Document)]
enum Foo {
    A { a: usize },
    B { b: usize },
}

#[derive(Document)]
enum Bar {
    C { c: usize },
    D { c: usize },
}

printer_test! {
    "output.commented.json" => to_json_fmt(Ty, {
         "enums_style": "Commented",
    }),

    "output.separated.json" => to_json_fmt(Ty, {
         "enums_style": "Separated",
    }),

    "output.without_key_quotes.commented.json" => to_json_fmt(Ty, {
        "objects_style": { "surround_keys_with_quotes": false },
        "enums_style": "Commented",
   }),

   "output.without_key_quotes.separated.json" => to_json_fmt(Ty, {
        "objects_style": { "surround_keys_with_quotes": false },
        "enums_style": "Separated",
   }),
}
