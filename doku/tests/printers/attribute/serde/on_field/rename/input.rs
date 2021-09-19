// run: test_json_ty

#[derive(Serialize, Doku)]
pub struct Ty {
    foo: String,

    #[serde(rename = "zar")]
    bar: String,
}
