use super::*;
use std::borrow::Cow;

impl Ctxt<'_, '_> {
    pub fn print_comment_for_scalar(&self, comment: &mut String) {
        let val = if let Some(val) = self.val {
            val
        } else {
            return;
        };

        let val = match val {
            val::Value::Bool(true) => Cow::Borrowed("true"),
            val::Value::Bool(false) => Cow::Borrowed("false"),
            val::Value::Float(val) => Cow::Owned(val.to_string()),
            val::Value::UnsignedInt(val) => Cow::Owned(val.to_string()),
            val::Value::SignedInt(val) => Cow::Owned(val.to_string()),
            val::Value::String(val) => Cow::Borrowed(val.as_str()),

            _ => {
                return;
            }
        };

        if !comment.is_empty() {
            write!(comment, "\n").unwrap();
        }

        write!(comment, "Default value: {}", val).unwrap();
    }
}
