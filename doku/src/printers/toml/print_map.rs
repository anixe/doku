use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_map(&mut self, key: &'ty Type, value: &'ty Type) {
        if !self.flat && self.should_indent() {
            self.out.inc_indent();
        }

        if let Some(example) = self.first_example() {
            self.out.write(example);
        } else {
            match key.kind {
                TypeKind::String => {
                    self.nested().with_ty(key).set_is_key().print();
                    self.out.write(" = ")
                }
                _ => panic!("Only String types are supported for map keys"),
            };
            self.nested().with_ty(value).print();
            self.out.ln();
            self.out.write("# ...");
        }

        if !self.flat && self.should_indent() {
            self.out.ln();
            self.out.dec_indent();
        }
    }
}
