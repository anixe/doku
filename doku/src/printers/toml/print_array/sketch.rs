mod expand_variants;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn sketch_array(&mut self, ty: &'ty Type, size: Option<usize>) {
        if self.is_table() {
            self.sketch_array_as_table(ty);
        } else {
            self.sketch_array_primitive(ty, size);
        }
    }

    fn sketch_array_as_table(&mut self, ty: &'ty Type) {
        let name = self.name.clone();
        let mut nested = self.nested().with_ty(ty).with_name(name);
        if !nested.try_expanding_variants(ty) {
            nested.print();
        }
    }

    fn sketch_array_primitive(&mut self, ty: &'ty Type, size: Option<usize>) {
        self.out.writeln("[");
        self.out.inc_indent();

        if self.try_expanding_variants(ty) {
            //
        } else if let Some(example) = self.example() {
            let examples: Vec<_> = example.iter().collect();

            for example in &examples {
                self.nested()
                    .with_ty(ty)
                    .with_example(Some(*example))
                    .print();

                self.out.writeln(",");

                // TODO if `size.is_some()` and this is the last example, the comma should not be printed
            }

            if size.map_or(true, |size| examples.len() < size) {
                self.out.writeln("# ...");
            }
        } else {
            self.nested().with_ty(ty).print();
            self.out.writeln(",");
            self.out.writeln("# ...");
        }

        self.out.dec_indent();
        self.out.write("]");
    }
}
