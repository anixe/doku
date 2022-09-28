use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_map(&mut self, key: &'ty Type, value: &'ty Type) {
        if !self.flat {
            self.out.writeln("{");
            self.out.inc_indent();
        }

        if let Some(example) = self.first_example() {
            self.out.write(example);
        } else {
            self.nested().with_ty(key).set_is_key().print();
            self.out.write(": ");
            self.nested().with_ty(value).print();
            self.out.write_property_separator_ln();
            self.out.write("/* ... */");
        }

        if !self.flat {
            self.out.ln();
            self.out.dec_indent();
            self.out.write("}");
        }
    }
}
