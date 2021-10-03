// run: to_json()

#[derive(Serialize, Document)]
#[serde(tag = "t")]
pub enum Ty {
    Foo { a: String },
    Bar { a: String },
}
