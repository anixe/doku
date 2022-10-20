use crate::prelude::*;

#[derive(Document)]
struct Struct {
    #[doku(meta(r#"fmt.enums_style = "Commented""#))]
    f1: Enum,

    #[doku(meta(r#"fmt.enums_style = "Separated""#))]
    f2: Enum,
}

#[derive(Document)]
enum Enum {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,

    /// This is `Zar`
    Zar,
}

printer_test! {
    "output.json" => to_json(Struct),
    "output.toml" => to_toml(Struct),
}
