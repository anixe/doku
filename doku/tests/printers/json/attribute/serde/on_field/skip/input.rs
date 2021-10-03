// run: to_json()

#[derive(Serialize, Document)]
pub struct Ty {
    foo: String,

    #[serde(skip)]
    bar: String,
}
