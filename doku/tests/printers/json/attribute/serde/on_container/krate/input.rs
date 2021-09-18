// run: to_json()

#[derive(Serialize, Document)]
#[serde(crate = "::serde")]
pub struct Ty {
    foo: String,
}
