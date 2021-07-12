use crate::*;

#[derive(Clone, Debug)]
pub struct Type {
    pub comment: Option<&'static str>,
    pub example: Option<&'static str>,

    /// When we have an adjacently-tagged enum, this field contains name of the
    /// field that should represent that enum's tag.
    ///
    /// E.g. in case of `ChangelogEvent` with its fancy `EventDetails`, we'd
    /// have `tag = Some("details");`.
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
    pub def: TypeDef,
}

impl Type {
    pub fn from_def(def: TypeDef) -> Self {
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
