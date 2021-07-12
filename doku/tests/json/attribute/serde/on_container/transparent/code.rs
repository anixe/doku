#[derive(Serialize, Doku)]
pub struct Ty {
    foo: Foo,
}

#[derive(Serialize, Doku)]
#[serde(transparent)]
pub struct Foo {
    bar: Bar,
}

#[derive(Serialize, Doku)]
pub struct Bar {
    f1: String,
    f2: String,
}
