#[derive(Doku)]
#[doku(tag = "t", content = "c")]
pub enum Ty {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}
