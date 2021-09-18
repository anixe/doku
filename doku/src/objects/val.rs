mod provider;

pub use self::provider::*;

#[derive(Debug)]
pub enum Value {
    Array { items: Vec<Value> },

    Bool(bool),

    // TODO(pwy) consider using separate types
    Float(f64),

    // TODO(pwy) consider using separate types
    UnsignedInt(u64),

    // TODO(pwy) consider using separate types
    SignedInt(i64),

    String(String),

    Struct { fields: Fields },
}

#[derive(Debug)]
pub enum Fields {
    Named { fields: Vec<(&'static str, Value)> },
    Unnamed { fields: Vec<Value> },
    Unit,
}
