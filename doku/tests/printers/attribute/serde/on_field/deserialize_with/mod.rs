use crate::prelude::*;

#[derive(Deserialize, Document)]
struct Ty {
    #[serde(deserialize_with = "fun")]
    foo: String,
}

fn fun<'de, D>(_: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    unimplemented!()
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
