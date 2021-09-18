mod def;
mod field;
mod fields;
mod provider;
mod tag;
mod variant;

pub use self::{def::*, field::*, fields::*, provider::*, tag::*, variant::*};

#[derive(Clone, Debug)]
pub struct Type {
    pub comment: Option<&'static str>,
    pub example: Option<&'static str>,

    /// When we have an adjacently-tagged enum, this field contains name of the
    /// field that should represent that enum's tag.
    pub tag: Option<&'static str>,

    /// Whether this type is serializable or not (think
    /// `#[serde(skip_serializing)]`). All types are serializable by
    /// default, which is a behavior consistent with Serde.
    pub serializable: bool,

    /// Whether this type is deserializable or not (think
    /// `#[serde(skip_deserializing)]`). All types are deserializable by
    /// default, which is a behavior consistent with Serde.
    pub deserializable: bool,

    // Keeping the definition last improves legibility of debug-printing
    pub def: Def,
}

impl Type {
    pub fn from_def(def: Def) -> Self {
        Self {
            comment: None,
            example: None,
            tag: None,
            serializable: true,
            deserializable: true,
            def,
        }
    }
}

impl From<Def> for Type {
    fn from(def: Def) -> Self {
        Self::from_def(def)
    }
}
