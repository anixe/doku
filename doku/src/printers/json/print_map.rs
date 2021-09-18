use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_map(&mut self, key: &'ty Type, value: &'ty Type) {
        if !self.flat {
            if self.inline {
                self.out.write("{ ");
            } else {
                self.out.writeln("{");
                self.out.inc_indent();
            }
        }

        if let Some(example) = self.example() {
            self.out.write(example);
        } else {
            self.nested().with_ty(key).print();
            self.out.write(": ");
            self.nested().with_ty(value).print();

            if self.inline {
                self.out.write(", ");
            } else {
                self.out.writeln(",");
            }

            self.out.write("/* ... */");
        }

        if !self.flat {
            if self.inline {
                self.out.write(" }");
            } else {
                self.out.ln();
                self.out.dec_indent();
                self.out.write("}");
            }
        }
    }
}
