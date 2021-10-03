/// Models the `#[doc]` attribute for doc-comments
pub struct Doc {
    pub comment: Option<String>,
}

impl Doc {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Self {
        let comments: Vec<_> = attrs
            .iter()
            .filter_map(|attr| {
                let (path, lit) =
                    if let syn::Meta::NameValue(syn::MetaNameValue {
                        path,
                        lit,
                        ..
                    }) = attr.parse_meta().ok()?
                    {
                        (path, lit)
                    } else {
                        return None;
                    };

                if path.get_ident()? != "doc" {
                    return None;
                }

                if let syn::Lit::Str(str) = lit {
                    Some(str.value())
                } else {
                    None
                }
            })
            .map(|doc| {
                // 99% of comments, including this one, begin with a space - we
                // want to get rid of it, because it makes the documentation
                // look ugly later.
                //
                // At the same time, we don't want to use `.trim()`, because if
                // somebody actually wrote `//   Foo`, they're probably using
                // those extra spaces to format the text.

                if let Some(doc) = doc.strip_prefix(' ') {
                    doc.to_string()
                } else {
                    doc
                }
            })
            .collect();

        let comment = comments.join("\n").trim().to_string();

        Self {
            comment: if comment.is_empty() {
                None
            } else {
                Some(comment)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::{parse_quote, ItemMod};

    mod given_no_docs {
        use super::*;

        #[test]
        fn returns_none() {
            let doc: ItemMod = parse_quote! {
                #[cfg(test)]
                #[foo = bar]
                mod module { }
            };

            let actual = Doc::from_ast(&doc.attrs).comment;
            let expected = None;

            assert_eq!(expected, actual);
        }
    }

    mod given_some_docs {
        use super::*;

        #[test]
        fn returns_them() {
            let doc: ItemMod = parse_quote! {
                #[cfg(test)]
                ///
                ///
                /// You say
                /// The price of my love’s not a price that you’re willing to pay
                /// You cry
                /// In your tea which you hurl in the sea when you see me go by
                ///
                /// Why so sad?
                ///  Remember we made an arrangement when you went away
                ///   Now you’re making me mad
                ///    Remember, despite our estrangement, I’m your man
                ///
                ///
                #[foo = bar]
                mod module { }
            };

            let actual = Doc::from_ast(&doc.attrs).comment;
            let actual = actual.as_deref();

            let expected = Some(indoc!(
                r#"
                You say
                The price of my love’s not a price that you’re willing to pay
                You cry
                In your tea which you hurl in the sea when you see me go by

                Why so sad?
                 Remember we made an arrangement when you went away
                  Now you’re making me mad
                   Remember, despite our estrangement, I’m your man"#
            ));

            assert_eq!(expected, actual);
        }
    }
}
