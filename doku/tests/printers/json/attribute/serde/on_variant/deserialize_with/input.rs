// run: to_json()

#[derive(Deserialize, Document)]
pub enum Ty {
    #[serde(deserialize_with = "fun")]
    Foo,
}

pub fn fun<'de, D>(_: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    unimplemented!()
}
