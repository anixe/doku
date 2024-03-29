use crate::*;

#[derive(Clone, Debug)]
pub struct Field {
    /// Type of this field
    pub ty: Type,

    /// Whether this field should get flattened (i.e. `#[serde(flatten)]`)
    pub flattened: bool,

    /// The serde aliases for this field
    pub aliases: &'static [&'static str],
}

impl From<Type> for Field {
    fn from(ty: Type) -> Self {
        Self {
            ty,
            flattened: false,
            aliases: &[],
        }
    }
}

impl From<TypeKind> for Field {
    fn from(kind: TypeKind) -> Self {
        let ty: Type = kind.into();
        ty.into()
    }
}

impl From<Fields> for Field {
    fn from(fields: Fields) -> Self {
        let kind: TypeKind = fields.into();
        kind.into()
    }
}
