mod serializer;

use self::serializer::*;
use crate::*;

#[derive(Debug)]
pub enum Value {
    /// Value of [`TypeKind::Optional`].
    None,

    /// Value of [`TypeKind::Bool`].
    Bool(bool),

    /// Value of [`TypeKind::String`].
    Char(char),

    /// Value of [`TypeKind::Float`].
    F32(f32),

    /// Value of [`TypeKind::Float`].
    F64(f64),

    /// Value of [`TypeKind::Integer`].
    U8(u8),

    /// Value of [`TypeKind::Integer`].
    I8(i8),

    /// Value of [`TypeKind::Integer`].
    U16(u16),

    /// Value of [`TypeKind::Integer`].
    I16(i16),

    /// Value of [`TypeKind::Integer`].
    U32(u32),

    /// Value of [`TypeKind::Integer`].
    I32(i32),

    /// Value of [`TypeKind::Integer`].
    U64(u64),

    /// Value of [`TypeKind::Integer`].
    I64(i64),

    /// Value of [`TypeKind::Integer`].
    U128(u128),

    /// Value of [`TypeKind::Integer`].
    I128(i128),

    /// Value of [`TypeKind::Integer`].
    Usize(usize),

    /// Value of [`TypeKind::Integer`].
    Isize(isize),

    /// Value of [`TypeKind::Integer`].
    String(String),

    /// Value of [`TypeKind::Array`] or [`TypeKind::Tuple`].
    Array(Vec<Value>),

    /// Value of [`TypeKind::Map`] or [`TypeKind::Struct`].
    Map(Vec<(Value, Value)>),
}

impl Value {
    pub fn as_struct_named_field(&self, name: &str) -> Option<&Self> {
        let fields = if let Self::Map(fields) = self {
            fields
        } else {
            return None;
        };

        fields.iter().find_map(|(name2, value)| {
            if let Value::String(name2) = name2 {
                if name2 == name {
                    return Some(value);
                }
            }

            None
        })
    }

    pub fn as_struct_unnamed_field(&self, idx: usize) -> Option<&Self> {
        match self {
            Self::Array(items) => items.get(idx),
            Self::Map(fields) => fields.get(idx).map(|(_, value)| value),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::None
    }
}

impl<S> From<&S> for Value
where
    S: Serialize,
{
    fn from(value: &S) -> Self {
        // Unwrap-safety: our serializer never actually panics
        value.serialize(ValueSerializer::default()).unwrap()
    }
}
