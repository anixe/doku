// run: test_json_ty

#[derive(Serialize, Doku)]
pub struct Ty {
    #[serde(default)]
    foo: bool,
}
