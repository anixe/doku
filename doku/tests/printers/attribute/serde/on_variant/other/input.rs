// run: test_json_ty

#[derive(Serialize, Doku)]
pub enum Ty {
    #[serde(other)]
    Foo,
}
