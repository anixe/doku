use super::*;

impl Ctxt<'_, '_> {
    pub fn print_comment_for_array(comment: &mut String, size: Option<usize>) {
        if let Some(size) = size {
            if comment.is_empty() {
                write!(comment, "Must").unwrap();
            } else {
                write!(comment, "; must").unwrap();
            }

            write!(comment, " contain exactly {} elements", size).unwrap();
        }
    }
}
