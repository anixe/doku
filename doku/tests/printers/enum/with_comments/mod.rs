use crate::prelude::*;

#[derive(Document)]
enum Ty {
    /// This...
    /// ... is `Foo`!
    Foo {
        /// Some comment
        /// (another line)
        /// (yet another line!)
        a: String,
    },

    /// This...
    /// ... is `Bar`!
    Bar { a: String, b: usize },
}

printer_test! {
    "output.json" => to_json_fmt(Ty, {
         "enums_style": "Commented",
    }),
}
