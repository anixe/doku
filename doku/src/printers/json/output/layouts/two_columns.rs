use super::*;

pub fn render(out: Output<'_>, align: bool, spacing: usize) -> String {
    let mut result = String::new();

    let left_col_max_width = out
        .lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    for Line {
        id: line_id,
        indent,
        body,
        comments,
    } in out.lines()
    {
        swrite!(result, if line_id > 0, "\n");
        swrite!(result, for 0..indent, " ");
        swrite!(result, "{}", body);

        if !comments.is_empty() {
            let left_col_width = if align {
                left_col_max_width
            } else {
                body.chars().count()
            } + spacing;

            swrite!(result, for body.chars().count()..left_col_width, " ");

            for (comment_id, comment) in comments.iter().enumerate() {
                if comment_id > 0 {
                    swrite!(result, "\n");
                    swrite!(result, for 0..(indent + left_col_width), " ");
                }

                swrite!(result, "// {}", comment);
            }
        }
    }

    result
}
