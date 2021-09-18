// run: to_json()

#[derive(Serialize, Document)]
pub struct Ty {
    #[serde(default)]
    foo: bool,
}
