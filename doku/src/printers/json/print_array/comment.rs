use super::*;

impl Ctxt<'_, '_, '_> {
    pub(super) fn comment_array(&mut self, size: Option<usize>) {
        if self.inline {
            return;
        }

        if !self.fmt.auto_comments.array_size {
            return;
        }

        let size = if let Some(size) = size {
            size
        } else {
            return;
        };

        self.out.append_comment(|comment| {
            if comment.is_empty() {
                swrite!(comment, "Must");
            } else {
                swrite!(comment, "; must");
            }

            swrite!(
                comment,
                " contain exactly {} element{}",
                size,
                if size == 1 { "" } else { "s" }
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target<T: Document>(array_size: bool) -> String {
        let fmt = Formatting {
            auto_comments: AutoComments {
                array_size,
                ..Default::default()
            },
            ..Default::default()
        };

        Printer::default().with_formatting(&fmt).print(&T::ty())
    }

    mod when_hint {
        use super::*;

        mod is_disabled {
            use super::*;

            #[test]
            fn then_doesnt_print_hint() {
                assert_doc!(
                    r#"
                    [
                      123,
                      /* ... */
                    ]
                    "#,
                    target::<[usize; 0]>(false)
                );
            }
        }

        mod is_enabled {
            use super::*;

            #[test]
            fn then_prints_hint() {
                assert_doc!(
                    r#"
                    // Must contain exactly 0 elements
                    [
                      123,
                      /* ... */
                    ]
                    "#,
                    target::<[usize; 0]>(true)
                );

                assert_doc!(
                    r#"
                    // Must contain exactly 1 element
                    [
                      123,
                      /* ... */
                    ]
                    "#,
                    target::<[usize; 1]>(true)
                );

                assert_doc!(
                    r#"
                    // Must contain exactly 128 elements
                    [
                      123,
                      /* ... */
                    ]
                    "#,
                    target::<[usize; 128]>(true)
                );
            }
        }
    }
}
