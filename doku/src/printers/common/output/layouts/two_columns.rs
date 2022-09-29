use super::*;

pub fn render<'a, O: 'a + Output<'a>>(
    out: &'a O,
    align: bool,
    spacing: usize,
) -> String {
    let mut result = String::new();

    let left_col_max_width = out
        .lines()
        .into_iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    for line in out.lines() {
        swrite!(result, if line.id > 0, "\n");
        swrite!(result, for 0..line.indent, " ");
        swrite!(result, "{}", line.body);

        if !line.comments.is_empty() {
            if align {
                swrite!(result, for line.len()..left_col_max_width, " ");
            }

            swrite!(result, for 0..spacing, " ");

            for (comment_id, comment) in line.comments.iter().enumerate() {
                if comment_id > 0 {
                    swrite!(result, "\n");

                    if align {
                        swrite!(result, for 0..left_col_max_width, " ");
                    } else {
                        swrite!(result, for 0..line.len(), " ");
                    }

                    swrite!(result, for 0..spacing, " ");
                }

                swrite!(result, "{} {}", out.comment_separator(), comment);
            }
        }
    }

    result
}
