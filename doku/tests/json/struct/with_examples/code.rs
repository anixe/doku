#[derive(Doku)]
pub struct Ty {
    /// Some comment
    #[doku(example = "foo-value")]
    foo: Option<String>,

    /// Another comment
    #[doku(example = "bar-value")]
    bar: Option<NestedString>,

    /// Yet another comment
    #[doku(example = "zar-value")]
    zar: Option<Option<Option<String>>>,
}

#[derive(Doku)]
struct NestedString(String);
