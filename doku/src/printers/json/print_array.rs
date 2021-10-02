mod comment;
mod sketch;

use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_array(&mut self, ty: &'ty Type, size: Option<usize>) {
        self.comment_array(size);
        self.sketch_array(ty, size);
    }
}
