#[derive(Serialize, Doku)]
pub enum Ty {
    Foo,

    #[serde(skip_serializing)]
    Bar,
}
