#[derive(Serialize, Doku)]
pub struct Ty {
    foo: String,

    #[serde(rename = "zar")]
    bar: String,
}
