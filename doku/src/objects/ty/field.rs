use crate::*;

#[derive(Clone, Debug)]
pub struct Field {
    /// Type of this field
    pub ty: ty::Type,

    /// Whether this field should get flattened (i.e. `#[serde(flatten)]`)
    pub flattened: bool,
}
