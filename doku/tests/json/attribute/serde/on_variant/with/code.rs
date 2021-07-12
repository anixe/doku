#[derive(Serialize, Deserialize, Doku)]
pub enum Ty {
    #[serde(with = "fun")]
    Foo,
}

pub mod fun {
    pub fn serialize<S>(_: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!()
    }

    pub fn deserialize<'de, D>(_: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
