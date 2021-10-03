// run: to_json()

#[derive(Deserialize, Document)]
pub enum Ty {
    Foo,

    #[serde(skip_deserializing)]
    Bar,
}
