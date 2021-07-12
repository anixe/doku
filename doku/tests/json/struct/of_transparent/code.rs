#[derive(Doku)]
pub struct Ty {
    /// This is `foo`
    foo: Foo,

    /// This is `bar`
    bar: Bar,

    /// This is `zar`
    zar: Zar,
}

#[derive(Doku)]
pub struct Foo {
    /// This is `f1`
    f1: String,

    /// This is `f2`
    f2: String,
}

#[derive(Doku)]
#[doku(transparent)]
pub struct Bar(BarNested);

#[derive(Doku)]
#[doku(transparent)]
pub struct BarNested {
    /// This is `f3`
    f3: String,
}

#[derive(Doku)]
pub struct Zar {
    /// This is `f4`
    f4: String,

    /// This is `f5`
    f5: String,
}
