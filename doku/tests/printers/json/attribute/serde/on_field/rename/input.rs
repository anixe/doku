// run: to_json()

#[derive(Serialize, Document)]
pub struct Ty {
    foo: String,

    #[serde(rename = "zar")]
    bar: String,
}
