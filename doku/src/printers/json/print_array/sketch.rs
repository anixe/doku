mod expand_variants;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn sketch_array(&mut self, ty: &'ty Type, size: Option<usize>) {
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
                self.out.writeln("/* ... */");
            }
        } else {
            self.nested().with_ty(ty).print();
            self.out.writeln(",");
            self.out.writeln("/* ... */");
        }

        self.out.dec_indent();
        self.out.write("]");
    }
}
