#[derive(Serialize, Doku)]
pub struct Ty {
    foo: String,

    #[serde(skip)]
    bar: String,
}
