// run: to_json()

#[derive(Document)]
pub struct Ty {
    /// This is `f1`
    f1: String,

    /// This is `foo`
    #[doku(flatten)]
    foo: Foo,

    /// This is `f4`
    f4: String,
}

#[derive(Document)]
#[doku(transparent)]
pub struct Foo {
    /// This is `bar`
    bar: Bar,
}

#[derive(Document)]
pub struct Bar {
    /// This is `f2`
    f2: String,

    /// This is `f3`
    f3: String,
}
