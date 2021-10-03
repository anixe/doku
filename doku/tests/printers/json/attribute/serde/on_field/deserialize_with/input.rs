// run: to_json()

#[derive(Deserialize, Document)]
pub struct Ty {
    #[serde(deserialize_with = "fun")]
    foo: String,
}

pub fn fun<'de, D>(_: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    unimplemented!()
}