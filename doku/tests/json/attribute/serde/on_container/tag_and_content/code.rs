#[derive(Serialize, Doku)]
#[serde(tag = "t", content = "c")]
pub enum Ty {
    Foo { a: String },
    Bar { a: String },
}
