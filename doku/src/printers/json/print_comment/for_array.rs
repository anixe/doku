use super::*;

impl Ctxt<'_, '_> {
    pub fn print_comment_for_array(comment: &mut String, size: Option<usize>) {
        if let Some(size) = size {
            if comment.is_empty() {
                comment.push_str(&format!("Must contain exactly {} elements", size));
            } else {
                comment.push_str(&format!("; must contain exactly {} elements", size));
            }
        }
    }
}
