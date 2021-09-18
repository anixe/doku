// run: to_json()

#[derive(Serialize, Document)]
#[serde(untagged)]
pub enum Ty {
    Foo { a: String },
    Bar { b: String },
}
