#[derive(Serialize, Doku)]
pub struct Ty {
    foo: String,

    #[serde(skip_serializing_if = "fun")]
    bar: String,
}

fn fun(_: &str) -> bool {
    unimplemented!()
}
