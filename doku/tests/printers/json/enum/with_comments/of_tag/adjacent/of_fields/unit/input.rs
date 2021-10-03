// run: to_json_fmt("output.commented.ty.json", json!({ "enums_style": "Commented" }))
// run: to_json_fmt("output.separated.ty.json", json!({ "enums_style": "Separated" }))

#[derive(Document)]
pub struct Ty {
    value: Enum,
}

#[derive(Document)]
#[doku(tag = "t", content = "c")]
pub enum Enum {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}
