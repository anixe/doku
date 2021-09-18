// run: to_json()

#[derive(Serialize, Document)]
pub struct Ty {
    #[serde(serialize_with = "fun")]
    foo: String,
}

pub fn fun<S>(_: &str, _: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unimplemented!()
}
