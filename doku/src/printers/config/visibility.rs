/// Defines the way skip_serializing/skip_deserializing are interpreted.
///
/// # Abstract
///
/// Serde allows for some fields to be skipped during the serialization and/or
/// deserialization phase.
///
/// Since Doku on its own doesn't know how given type is going to be used (i.e.
/// whether it'll be serialized or deserialized), it cannot just take a guess -
/// you have to explicitly tell the type printer what's the case.
///
/// If you don't know or don't care, use `Visibility::default()` - this will
/// render both the serializable and the deserializable fields.
#[derive(Clone, Copy, Debug)]
pub enum Visibility {
    All,
    SerializableOnly,
    DeserializableOnly,
}

impl Visibility {
    pub(crate) fn allows(
        self,
        is_serializable: bool,
        is_deserializable: bool,
    ) -> bool {
        match self {
            Self::All => true,
            Self::SerializableOnly => is_serializable,
            Self::DeserializableOnly => is_deserializable,
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Self::All
    }
}
