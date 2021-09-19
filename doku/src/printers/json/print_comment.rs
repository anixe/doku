mod for_array;
mod for_enum;
mod for_optional;
mod for_scalar;

use super::*;

impl Ctxt<'_, '_> {
    /// This function is responsible for printing nearly all comments that are
    /// eventually visible on the right side of the documentation.
    ///
    /// It may seem like a fairly simple task, but it's actually quite hard to
    /// get it right, especially when it comes to enumeration types.
    pub fn print_comment(&mut self) {
        // We cannot print comments while being in the inline mode, because
        // there's no space for them; for reference, please see docs for the
        // `.inline` field
        if self.inline {
            return;
        }

        let mut comment = String::new();

        // User-made comment always takes precedence over the machine-generated
        // ones, so let's start with it
        if let Some(user_comment) = &self.ty.comment {
            comment.push_str(user_comment);
        }

        // ... and now let's try to add a machine-generated comment
        match &self.ty.def {
            ty::Def::Array { size, .. } => {
                Self::print_comment_for_array(&mut comment, *size);
            }

            ty::Def::Enum { tag, variants } => {
                self.print_comment_for_enum(comment, *tag, variants);

                // Contrary to the other comment-printers, this one invokes
                // `self.out.comment()` on its own, so we have to return earlier
                // in this case not to duplicate the comment
                return;
            }

            ty::Def::Optional { .. } => {
                self.print_comment_for_optional(&mut comment);
            }

            ty::Def::Bool | ty::Def::Float | ty::Def::Integer | ty::Def::String => {
                self.print_comment_for_scalar(&mut comment);
            }

            _ => (),
        }

        if !comment.is_empty() {
            for comment in comment.split('\n') {
                self.out.comment(format!("// {}", comment));
            }
        }
    }
}
