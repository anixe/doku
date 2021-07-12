use super::*;

impl Ctxt<'_, '_> {
    pub fn print_comment_for_optional(&self, comment: &mut String) {
        if let Some(parent) = self.parents.last() {
            // Avoid printing `Optional` twice (e.g. for `Option<Option<String>>`)
            if matches!(parent.def, TypeDef::Optional { .. }) {
                return;
            }
        }

        if comment.is_empty() {
            comment.push_str("Optional");
        } else {
            comment.push_str("; optional");
        }
    }
}
