use super::*;

impl<'ty> Ctxt<'_, 'ty, '_> {
    pub(super) fn print_optional(&mut self, ty: &'ty Type) {
        self.comment_optional();
        self.sketch_optional(ty);
    }

    fn comment_optional(&mut self) {
        if self.inline {
            return;
        }

        if !self.fmt.auto_comments.optional {
            return;
        }

        // Avoid printing hint twice (e.g. for `Option<Option<String>>`)
        if let Some(Type {
            kind: TypeKind::Optional { .. },
            ..
        }) = self.parent
        {
            return;
        }

        self.out.append_comment(|comment| {
            if comment.is_empty() {
                swrite!(comment, "Optional");
            } else {
                swrite!(comment, "; optional");
            }
        });
    }

    fn sketch_optional(&mut self, ty: &'ty Type) {
        let example = self.example();

        self.nested().with_ty(ty).with_example(example).print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target<T: Document>(optional: bool) -> String {
        let fmt = Formatting {
            auto_comments: AutoComments {
                optional,
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
                    123
                    "#,
                    target::<Option<usize>>(false)
                );
            }
        }

        mod is_enabled {
            use super::*;

            #[test]
            fn then_prints_hint() {
                assert_doc!(
                    r#"
                    // Optional
                    123
                    "#,
                    target::<Option<usize>>(true)
                );

                assert_doc!(
                    r#"
                    // Optional
                    123
                    "#,
                    target::<Option<Option<Option<usize>>>>(true)
                );
            }
        }
    }
}
