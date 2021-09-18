// run: to_json()

#[derive(Document)]
pub struct Ty {
    /// This is `foo`
    foo: Foo,

    /// This is `bar`
    #[doku(flatten)]
    bar: Bar,

    /// This is `zar`
    zar: Zar,
}

#[derive(Document)]
pub struct Foo {
    f1: String,
    f2: String,
}

#[derive(Document)]
pub struct Bar {
    f3: String,
    f4: String,
    f5: BarNested,
}

#[derive(Document)]
pub struct BarNested {
    f6: String,
}

#[derive(Document)]
pub struct Zar {
    f7: String,
    f8: String,
}
