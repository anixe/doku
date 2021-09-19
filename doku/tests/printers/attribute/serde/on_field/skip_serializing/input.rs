// run: test_json_ty

#[derive(Serialize, Doku)]
pub struct Ty {
    foo: String,

    #[serde(skip_serializing)]
    bar: String,
}
