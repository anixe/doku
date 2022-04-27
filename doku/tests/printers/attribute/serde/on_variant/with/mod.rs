use crate::prelude::*;

#[derive(Serialize, Deserialize, Document)]
enum Ty {
    #[serde(with = "fun")]
    Foo,
}

mod fun {
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

printer_test! {
    "output.json" => to_json(Ty),
}
