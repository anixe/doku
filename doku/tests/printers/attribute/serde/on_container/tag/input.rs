// run: test_json_ty

#[derive(Serialize, Doku)]
#[serde(tag = "t")]
pub enum Ty {
    Foo { a: String },
    Bar { a: String },
}
