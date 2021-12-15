// run: to_json()

#[derive(Document)]
pub struct Ty {
    /// Comment for f1
    #[doku(literal_example = r#"[ "aaa", "bbb" ]"#)]
    f1: String,
}
