use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_map(&mut self, key: &'ty ty::Type, value: &'ty ty::Type) {
        if !self.flat {
            if self.inline {
                self.out.text("{ ");
            } else {
                self.out.line("{");
                self.out.inc_indent();
            }
        }

        if let Some(example) = self.example() {
            self.out.text(example);
        } else {
            self.with_ty(key).print();
            self.out.text(": ");
            self.with_ty(value).print();

            if self.inline {
                self.out.text(", ");
            } else {
                self.out.line(",");
            }

            self.out.text("/* ... */");
        }

        if !self.flat {
            if self.inline {
                self.out.text(" }");
            } else {
                self.out.newline();
                self.out.dec_indent();
                self.out.text("}");
            }
        }
    }
}
