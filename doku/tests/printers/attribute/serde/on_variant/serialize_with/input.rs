// run: test_json_ty

#[derive(Serialize, Doku)]
pub enum Ty {
    #[serde(serialize_with = "fun")]
    Foo,
}

pub fn fun<S>(_: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    unimplemented!()
}
