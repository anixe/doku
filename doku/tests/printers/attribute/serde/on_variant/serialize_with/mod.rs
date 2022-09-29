use crate::prelude::*;

#[derive(Serialize, Document)]
enum Ty {
    #[serde(serialize_with = "fun")]
    Foo,
}

fn fun<S>(_: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unimplemented!()
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(Ty),
}
