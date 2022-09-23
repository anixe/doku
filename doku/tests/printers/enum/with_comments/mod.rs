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

    "output.without_key_quotes.json" => to_json_fmt(Ty, {
        "objects_style": { "surround_keys_with_quotes": false },
        "enums_style": "Commented",
   }),
}
