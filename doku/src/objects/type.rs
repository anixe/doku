use crate::*;

#[derive(Clone, Debug)]
pub struct Type {
    pub comment: Option<&'static str>,
    pub example: Option<Example>,
    pub metas: Metas,

    /// When we have an adjacently-tagged enum, this field contains name of the
    /// field that should represent that enum's tag.
    pub tag: Option<&'static str>,

    /// Whether this type is serializable or not (think
    /// `#[serde(skip_serializing)]`).
    pub serializable: bool,

    /// Whether this type is deserializable or not (think
    /// `#[serde(skip_deserializing)]`).
    pub deserializable: bool,

    // Keeping the kind last improves legibility of debug-printing
    pub kind: TypeKind,
}

impl From<TypeKind> for Type {
    fn from(kind: TypeKind) -> Self {
        Self {
            comment: None,
            example: None,
            metas: Metas::default(),
            tag: None,
            serializable: true,
            deserializable: true,
            kind,
        }
    }
}
