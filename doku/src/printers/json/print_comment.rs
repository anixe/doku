use super::*;

impl Ctxt<'_, '_, '_> {
    pub(super) fn print_comment(&mut self) {
        if !self.inline {
            if let DocComments::Visible = self.fmt.doc_comments {
                if let Some(comment) = &self.ty.comment {
                    self.out.writeln_comment(comment);
                }
            }
        }
    }
}
