use super::*;

pub fn expand_variant(
    variant: &syn::Variant,
    rename_variants: RenameRule,
) -> Result<TokenStream2> {
    let syn::Variant {
        attrs,
        ident,
        fields,
        ..
    } = variant;

    let doku = attrs::DokuVariant::from_ast(attrs)?;
    let serde = attrs::SerdeVariant::from_ast(attrs)?;

    let ident = rename_variants.apply_to_variant(&ident.to_string());

    let rename_fields =
        doku.rename_all.or(serde.rename_all).unwrap_or_default();

    let mut variant = Variant {
        id: quote! { #ident },
        title: quote! { #ident },
        comment: quote! { None },
        fields: expand_fields(fields, rename_fields)?,
        serializable: true,
        deserializable: true,
    };

    variant.add_doc_attrs(attrs);
    variant.add_serde_attrs(&attrs)?;
    variant.add_doku_attrs(&attrs)?;

    Ok(variant.render())
}

struct Variant {
    id: TokenStream2,
    title: TokenStream2,
    comment: TokenStream2,
    fields: TokenStream2,
    serializable: bool,
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
            alias: _,
            deserialize_with: _,
            other: _,
            rename,
            rename_all: _,
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
        let attrs::DokuVariant {
            rename,
            rename_all: _,
            skip,
        } = attrs::DokuVariant::from_ast(&attrs)?;

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
