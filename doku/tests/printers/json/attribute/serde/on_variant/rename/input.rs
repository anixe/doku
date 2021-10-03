// run: to_json()

#[derive(Serialize, Document)]
pub enum Ty {
    #[serde(rename = "Bar")]
    Foo,
}
