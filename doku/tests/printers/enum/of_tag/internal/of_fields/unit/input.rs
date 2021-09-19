// run: test_json_ty

#[derive(Doku)]
#[doku(tag = "t")]
pub enum Ty {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}
