use crate::*;

#[derive(Clone, Debug)]
pub enum Def {
    /// A homogeneous array of a possibly known size
    Array {
        /// Type of items this array accepts
        ty: Box<ty::Type>,

        /// An optional hint about the array's expected size
        size: Option<usize>,
    },

    /// `true` / `false`
    Bool,

    /// An algebraic data type
    Enum {
        /// The way enum should be serialized
        tag: ty::Tag,

        /// All enum's variants
        variants: Vec<ty::Variant>,
    },

    /// A floating-point number
    Float,

    /// An integer number
    Integer,

    /// A homogeneous map
    Map { key: Box<ty::Type>, value: Box<ty::Type> },

    /// `Option<Ty>`
    Optional { ty: Box<ty::Type> },

    /// A UTF-8 string
    String,

    /// A structure
    Struct {
        fields: ty::Fields,

        /// Whether this type should behave as a silent-wrapper.
        /// Corresponds to `#[serde(transparent)]`.
        transparent: bool,
    },

    /// A heterogeneous list of an up-front known size
    Tuple { fields: Vec<ty::Type> },
}
