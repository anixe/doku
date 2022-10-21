use crate::Value;

pub fn value_to_string(val: &Value) -> Option<String> {
    let escape_char = |val: char| match val {
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\n' => "\\n".to_string(),
        '\\' => "\\\\".to_string(),
        val => val.to_string(),
    };

    let escape_string = |val: &str| {
        let val: String = val
            .chars()
            .map(escape_char)
            .flat_map(|str| str.chars().collect::<Vec<_>>())
            .collect();

        format!(r#""{}""#, val)
    };

    match val {
        Value::Bool(val) => Some(val.to_string()),
        Value::Char(val) => Some(escape_char(*val)),
        Value::F32(val) => Some(val.to_string()),
        Value::F64(val) => Some(val.to_string()),
        Value::U8(val) => Some(val.to_string()),
        Value::I8(val) => Some(val.to_string()),
        Value::U16(val) => Some(val.to_string()),
        Value::I16(val) => Some(val.to_string()),
        Value::U32(val) => Some(val.to_string()),
        Value::I32(val) => Some(val.to_string()),
        Value::U64(val) => Some(val.to_string()),
        Value::I64(val) => Some(val.to_string()),
        Value::U128(val) => Some(val.to_string()),
        Value::I128(val) => Some(val.to_string()),
        Value::Usize(val) => Some(val.to_string()),
        Value::Isize(val) => Some(val.to_string()),
        Value::String(val) => Some(escape_string(val)),
        Value::Array(_) => None,
        Value::Map(_) => None,
        Value::None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Value::Bool(false) => Some("false".to_string()) ; "bool, false")]
    #[test_case(Value::Bool(true) => Some("true".to_string()) ; "bool, true")]
    #[test_case(Value::Char('\t') => Some("\\t".to_string()) ; "char, \\t")]
    #[test_case(Value::Char('\r') => Some("\\r".to_string()) ; "char, \\r")]
    #[test_case(Value::Char('\n') => Some("\\n".to_string()) ; "char, \\n")]
    #[test_case(Value::Char('\\') => Some("\\\\".to_string()) ; "char, \\")]
    #[test_case(Value::Char('a') => Some("a".to_string()) ; "char, a")]
    #[test_case(Value::F32(1.23) => Some("1.23".to_string()) ; "f32")]
    #[test_case(Value::F64(1.23) => Some("1.23".to_string()) ; "f64")]
    #[test_case(Value::U8(123) => Some("123".to_string()) ; "u8")]
    #[test_case(Value::I8(123) => Some("123".to_string()) ; "i8")]
    #[test_case(Value::U16(123) => Some("123".to_string()) ; "u16")]
    #[test_case(Value::I16(123) => Some("123".to_string()) ; "i16")]
    #[test_case(Value::U32(123) => Some("123".to_string()) ; "u32")]
    #[test_case(Value::I32(123) => Some("123".to_string()) ; "i32")]
    #[test_case(Value::U64(123) => Some("123".to_string()) ; "u64")]
    #[test_case(Value::I64(123) => Some("123".to_string()) ; "i64")]
    #[test_case(Value::U128(123) => Some("123".to_string()) ; "u128")]
    #[test_case(Value::I128(123) => Some("123".to_string()) ; "i128")]
    #[test_case(Value::Usize(123) => Some("123".to_string()) ; "usize")]
    #[test_case(Value::Isize(123) => Some("123".to_string()) ; "isize")]
    #[test_case(Value::String("Hello \n World!".to_string()) => Some("\"Hello \\n World!\"".to_string()) ; "string")]
    #[test_case(Value::Array(Default::default()) => None ; "array")]
    #[test_case(Value::Map(Default::default()) => None ; "map")]
    #[test_case(Value::None => None ; "none")]
    fn returns(val: Value) -> Option<String> {
        value_to_string(&val)
    }
}
