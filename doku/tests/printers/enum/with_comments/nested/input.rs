// run: test_json_ty

#[derive(Doku)]
pub struct Ty {
    /// This is `en`
    en: Enum,
}

#[derive(Doku)]
pub enum Enum {
    /// This is `foo`
    Foo,

    /// This is `bar`
    Bar,
}
