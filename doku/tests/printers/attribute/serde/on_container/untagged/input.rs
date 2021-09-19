// run: test_json_ty

#[derive(Serialize, Doku)]
#[serde(untagged)]
pub enum Ty {
    Foo { a: String },
    Bar { b: String },
}
