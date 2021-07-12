/// Defines the way `#[serde(skip_serializing/deserializing)]` are be
/// interpreted.
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
/// If you don't know or don't care, use `TypePrinterMode::default()` - this
/// will render both the serializable, and the deserializable fields.
#[derive(Clone, Copy, Debug)]
pub enum TypePrinterMode {
    All,
    SerializableOnly,
    DeserializableOnly,
}

impl TypePrinterMode {
    crate fn allows(self, is_type_serializable: bool, is_type_deserializable: bool) -> bool {
        match self {
            Self::All => true,
            Self::SerializableOnly => is_type_serializable,
            Self::DeserializableOnly => is_type_deserializable,
        }
    }
}

impl Default for TypePrinterMode {
    fn default() -> Self {
        Self::All
    }
}
