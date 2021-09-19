// run: test_json_ty

#[derive(Serialize, Deserialize, Doku)]
pub struct Ty {
    #[serde(with = "fun")]
    foo: String,
}

pub mod fun {
    pub fn serialize<S>(_: &str, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!()
    }

    pub fn deserialize<'de, D>(_: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
