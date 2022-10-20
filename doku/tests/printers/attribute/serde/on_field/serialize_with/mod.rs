use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    #[serde(serialize_with = "fun")]
    foo: String,
}

fn fun<S>(_: &str, _: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unimplemented!()
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
