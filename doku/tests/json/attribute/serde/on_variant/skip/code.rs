#[derive(Serialize, Doku)]
pub enum Ty {
    Foo,

    #[serde(skip)]
    Bar,
}
