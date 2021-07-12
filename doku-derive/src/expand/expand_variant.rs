use super::*;

/// Expands roughly to:
///
/// ```ignore
/// ::doku::Variant {
///     id: ...,
///     title: ...,
///     comment: ...,
///     serializable: ...,
///     deserializable: ...,
///     fields: ...,
/// },
/// ```
pub fn expand_variant(variant: &syn::Variant) -> Result<TokenStream2> {
    let syn::Variant {
        attrs, ident, fields, ..
    } = variant;

    let mut variant = Variant {
        id:             quote! { stringify!(#ident) },
        title:          quote! { stringify!(#ident) },
        comment:        quote! { None },
        fields:         expand_fields(fields)?,
        serializable:   true,
        deserializable: true,
    };

    variant.add_doc_attrs(attrs);
    variant.add_serde_attrs(&attrs)?;
    variant.add_doku_attrs(&attrs)?;

    Ok(variant.render())
}

struct Variant {
    id:             TokenStream2,
    title:          TokenStream2,
    comment:        TokenStream2,
    fields:         TokenStream2,
    serializable:   bool,
    deserializable: bool,
}

impl Variant {
    fn add_doc_attrs(&mut self, attrs: &[syn::Attribute]) {
        if let Some(comment) = attrs::Doc::from_ast(attrs).comment {
            self.comment = quote! { Some(#comment) };
        }
    }

    fn add_serde_attrs(&mut self, attrs: &[syn::Attribute]) -> Result<()> {
        let attrs::SerdeVariant {
            deserialize_with: _,
            other: _,
            rename,
            serialize_with: _,
            skip,
            skip_deserializing,
            skip_serializing,
            with: _,
        } = attrs::SerdeVariant::from_ast(attrs)?;

        if let Some(val) = rename {
            self.id = quote_spanned! { val.span() => #val };
        }

        if let Some(val) = skip {
            self.serializable = !val;
            self.deserializable = !val;
        }

        if let Some(val) = skip_deserializing {
            self.deserializable = !val;
        }

        if let Some(val) = skip_serializing {
            self.serializable = !val;
        }

        Ok(())
    }

    fn add_doku_attrs(&mut self, attrs: &[syn::Attribute]) -> Result<()> {
        let attrs::DokuVariant { rename, skip } = attrs::DokuVariant::from_ast(&attrs)?;

        if let Some(val) = rename {
            self.id = quote_spanned! { val.span() => #val };
        }

        if let Some(val) = skip {
            self.serializable = !val;
            self.deserializable = !val;
        }

        Ok(())
    }

    fn render(self) -> TokenStream2 {
        let Self {
            id,
            title,
            comment,
            serializable,
            deserializable,
            fields,
        } = self;

        if serializable || deserializable {
            quote! {
                ::doku::Variant {
                    id: #id,
                    title: #title,
                    comment: #comment,
                    serializable: #serializable,
                    deserializable: #deserializable,
                    fields: #fields,
                },
            }
        } else {
            quote! {
                //
            }
        }
    }
}
