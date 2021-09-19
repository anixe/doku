// run: test_json_ty

#[derive(Serialize, Doku)]
#[serde(crate = "serde")]
pub struct Ty {
    foo: String,
}
