mod provider;

pub use self::provider::*;

#[derive(Debug)]
pub enum Value {
    Array { items: Vec<Value> },

    Bool(bool),

    // TODO consider using separate types
    Float(f64),

    // TODO consider using separate types
    UnsignedInt(u64),

    // TODO consider using separate types
    SignedInt(i64),

    String(String),

    Struct { fields: Fields },
}

impl Value {
    pub fn as_struct_named_field(&self, name: &str) -> Option<&Self> {
        let fields = if let Self::Struct {
            fields: Fields::Named { fields },
        } = self
        {
            fields
        } else {
            return None;
        };

        fields.iter().find(|(name2, _)| *name2 == name).map(|(_, value)| value)
    }

    pub fn as_struct_unnamed_field(&self, idx: usize) -> Option<&Self> {
        let fields = if let Self::Struct {
            fields: Fields::Unnamed { fields },
        } = self
        {
            fields
        } else {
            return None;
        };

        fields.get(idx)
    }
}

#[derive(Debug)]
pub enum Fields {
    Named { fields: Vec<(&'static str, Value)> },
    Unnamed { fields: Vec<Value> },
    Unit,
}
