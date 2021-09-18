// run: to_json()

#[derive(Serialize, Document)]
pub struct Ty {
    foo: Foo,
}

#[derive(Serialize, Document)]
#[serde(transparent)]
pub struct Foo {
    bar: Bar,
}

#[derive(Serialize, Document)]
pub struct Bar {
    f1: String,
    f2: String,
}
