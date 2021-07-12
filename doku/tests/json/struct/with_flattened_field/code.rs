#[derive(Doku)]
pub struct Ty {
    /// This is `foo`
    foo: Foo,

    /// This is `bar`
    #[doku(flatten)]
    bar: Bar,

    /// This is `zar`
    zar: Zar,
}

#[derive(Doku)]
pub struct Foo {
    f1: String,
    f2: String,
}

#[derive(Doku)]
pub struct Bar {
    f3: String,
    f4: String,
    f5: BarNested,
}

#[derive(Doku)]
pub struct BarNested {
    f6: String,
}

#[derive(Doku)]
pub struct Zar {
    f7: String,
    f8: String,
}
