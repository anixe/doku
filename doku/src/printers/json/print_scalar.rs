use super::*;

impl Ctxt<'_, '_, '_> {
    pub(super) fn print_bool(&mut self) {
        self.print_scalar(self.example().unwrap_or("true"));
    }

    pub(super) fn print_float(&mut self) {
        self.print_scalar(self.example().unwrap_or("123.45"));
    }

    pub(super) fn print_integer(&mut self) {
        self.print_scalar(self.example().unwrap_or("123"));
    }

    pub(super) fn print_string(&mut self) {
        self.print_scalar(&format!(
            "\"{}\"",
            self.example().unwrap_or("string")
        ));
    }

    fn print_scalar(&mut self, val: &str) {
        self.comment_scalar();
        self.sketch_scalar(val);
    }

    fn comment_scalar(&mut self) {
        let prefix =
            if let ValuesStyle::Comment(prefix) = &self.fmt.values_style {
                prefix
            } else {
                return;
            };

        let val = if let Some(val) = self.val.and_then(Self::value_to_string) {
            val
        } else {
            return;
        };

        self.out.append_comment(|comment| {
            swrite!(comment, if !comment.is_empty(), "\n");
            swrite!(comment, "{}{}", prefix, val);
        });
    }

    fn sketch_scalar(&mut self, val: &str) {
        if let ValuesStyle::Field = self.fmt.values_style {
            if let Some(val) = self.val.and_then(Self::value_to_string) {
                self.out.write(val);
                return;
            }
        }

        self.out.write(val);
    }
}
