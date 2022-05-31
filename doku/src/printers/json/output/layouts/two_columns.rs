use super::*;

pub fn render(out: Output, align: bool, spacing: usize) -> String {
    let mut result = String::new();

    let left_col_max_width =
        out.lines().map(|line| line.len()).max().unwrap_or(0);

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

                swrite!(result, "// {}", comment);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod align {
        use super::*;

        #[test]
        fn with_nested_indents() {
            let fmt = Formatting {
                layout: Layout::TwoColumns {
                    align: true,
                    spacing: 1,
                },
                ..Default::default()
            };

            let mut out = Output::new(&fmt);

            out.write("1");
            out.writeln_comment("2");
            out.ln();
            out.inc_indent();
            out.write("3");
            out.writeln_comment("4");
            out.ln();
            out.inc_indent();
            out.write("5");
            out.writeln_comment("6");
            out.ln();
            out.dec_indent();
            out.dec_indent();

            assert_doc!(
                r#"
                1     // 2
                  3   // 4
                    5 // 6
                "#,
                out.render()
            );
        }
    }
}
