// run: test_json_ty

#[derive(Doku)]
pub enum Ty {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}
