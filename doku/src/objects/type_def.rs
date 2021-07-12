use super::*;

#[derive(Clone, Debug)]
pub enum TypeDef {
    /// A homogeneous array of a possibly known size
    Array {
        /// Type of items this array accepts
        ty: Box<Type>,

        /// An optional hint about the array's expected size
        size: Option<usize>,
    },

    /// `true` / `false`
    Bool,

    /// An algebraic data type
    Enum {
        /// The way enum should be serialized
        tag: Tag,

        /// All enum's variants
        variants: Vec<Variant>,
    },

    /// A floating-point number
    Float,

    /// An integer number
    Integer,

    /// A homogeneous map
    Map { key: Box<Type>, value: Box<Type> },

    /// `Option<Ty>`
    Optional { ty: Box<Type> },

    /// A UTF-8 string
    String,

    /// A structure
    Struct {
        fields: Fields,

        /// Whether this type should behave as a silent-wrapper.
        /// Corresponds to `#[serde(transparent)]`.
        transparent: bool,
    },

    /// A heterogeneous list of an up-front known size
    Tuple { fields: Vec<Type> },
}
