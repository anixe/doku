#[derive(Deserialize, Doku)]
pub enum Ty {
    Foo,

    #[serde(skip_deserializing)]
    Bar,
}
