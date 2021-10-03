// run: to_json()

#[derive(Serialize, Document)]
pub enum Ty {
    Foo,

    #[serde(skip)]
    Bar,
}
