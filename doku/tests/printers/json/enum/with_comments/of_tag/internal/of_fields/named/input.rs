// run: to_json_fmt("output.commented.ty.json", json!({ "enums_style": "Commented" }))
// run: to_json_fmt("output.separated.ty.json", json!({ "enums_style": "Separated" }))

#[derive(Document)]
pub struct Ty {
    value: Enum,
}

#[derive(Document)]
#[doku(tag = "t")]
pub enum Enum {
    /// This is `Foo`
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}
