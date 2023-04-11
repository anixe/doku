use crate::*;

#[derive(Clone, Debug)]
pub struct Variant {
    /// Identifier of the variant; it includes `#[serde(rename)]` and similar
    /// attributes, so this string is exactly what gets serialized into the
    /// output.
    pub id: &'static str,

    /// Title of the variant as it was written in the Rust code, excluding
    /// stuff like `#[serde(rename)]`.
    pub title: &'static str,

    pub comment: Option<&'static str>,
    pub serializable: bool,
    pub deserializable: bool,
    pub fields: Fields,
    pub aliases: &'static [&'static str],
}
