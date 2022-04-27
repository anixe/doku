use crate::prelude::*;

#[derive(Serialize, Deserialize, Document)]
struct Ty {
    #[serde(with = "fun")]
    foo: String,
}

mod fun {
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

printer_test! {
    "output.json" => to_json(Ty),
}
