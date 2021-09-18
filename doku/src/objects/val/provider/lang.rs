use crate::*;

impl val::Provider for bool {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::Bool(*self as _))
    }
}

// ---

impl val::Provider for u8 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::UnsignedInt(*self as _))
    }
}

impl val::Provider for u16 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::UnsignedInt(*self as _))
    }
}

impl val::Provider for u32 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::UnsignedInt(*self as _))
    }
}

impl val::Provider for u64 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::UnsignedInt(*self as _))
    }
}

impl val::Provider for usize {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::UnsignedInt(*self as _))
    }
}

// ---

impl val::Provider for i8 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::SignedInt(*self as _))
    }
}

impl val::Provider for i16 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::SignedInt(*self as _))
    }
}

impl val::Provider for i32 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::SignedInt(*self as _))
    }
}

impl val::Provider for i64 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::SignedInt(*self as _))
    }
}

impl val::Provider for isize {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::SignedInt(*self as _))
    }
}

// ---

impl val::Provider for f32 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::Float(*self as _))
    }
}

impl val::Provider for f64 {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::Float(*self as _))
    }
}

// ---

impl val::Provider for &str {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::String(self.to_string()))
    }
}
