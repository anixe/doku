use crate::*;

/// Defines a single enum's variant.
///
/// For instance, given this enum:
///
/// ```rust
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// enum Subject {
///     /// Picture of a cat
///     #[serde(rename = "picture-of-cat")]
///     PictureOfCat,
/// }
/// ```
///
/// ... the `Subject::PictureOfCat` variant is defined as:
///
/// ```rust
/// # use doku::ty::{Fields, Variant};
/// # let _ =
/// Variant {
///     id: "picture-of-cat",
///     title: "PictureOfCat",
///     comment: Some("Picture of a cat"),
///     serializable: true,
///     deserializable: true,
///     fields: Fields::Unit,
/// }
/// # ;
/// ```
///
/// When no `#[serde(rename = ...)]` (or similar) has been provided, `id` is
/// equal to `title`.
#[derive(Clone, Debug)]
pub struct Variant {
    /// Identifier of the variant; it includes `#[serde(rename)]` and similar
    /// attributes, so this string is exactly what gets serialized into the
    /// output.
    pub id: &'static str,

    /// Title of the variant as it was written in the Rust code, excluding stuff
    /// like `#[serde(rename)]`.
    pub title: &'static str,

    pub comment:        Option<&'static str>,
    pub serializable:   bool,
    pub deserializable: bool,
    pub fields:         ty::Fields,
}
