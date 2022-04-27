use crate::prelude::*;

#[derive(Deserialize, Document)]
enum Ty {
    #[serde(deserialize_with = "fun")]
    Foo,
}

fn fun<'de, D>(_: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    unimplemented!()
}

printer_test! {
    "output.json" => to_json(Ty),
}
