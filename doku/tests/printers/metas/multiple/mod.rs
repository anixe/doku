use crate::prelude::*;

#[derive(Document)]
struct Ty1 {
    #[doku(meta(r#"fmt.doc_comments = "Visible""#))]
    #[doku(meta(r#"fmt.auto_comments.optional = true"#))]
    foo: Foo,
}

#[derive(Document)]
struct Ty2 {
    #[doku(meta(r#"fmt.doc_comments = "Hidden""#))]
    #[doku(meta(r#"fmt.auto_comments.optional = false"#))]
    foo: Foo,
}

#[derive(Document)]
struct Foo {
    /// This is `Bar`
    bar: Option<String>,
}

printer_test! {
    "output.ty1.json" => to_json(Ty1),
    "output.ty2.json" => to_json(Ty2),
    "output.ty1.toml" => to_toml(Ty1),
    "output.ty2.toml" => to_toml(Ty2),
}
