use super::*;

pub fn expand_field(field: &syn::Field, named: bool) -> Result<TokenStream2> {
    let syn::Field { attrs, ident, ty, .. } = field;

    let mut field = Field {
        name:           quote! { stringify!(#ident) },
        ty:             quote! { #ty },
        comment:        quote! { None },
        example:        quote! { None },
        tag:            quote! { None },
        serializable:   true,
        deserializable: true,
        flattened:      false,
    };

    field.add_doc_attrs(attrs);
    field.add_serde_attrs(&attrs)?;
    field.add_doku_attrs(&attrs)?;

    Ok(field.render(named))
}

struct Field {
    name:           TokenStream2,
    ty:             TokenStream2,
    comment:        TokenStream2,
    example:        TokenStream2,
    tag:            TokenStream2,
    serializable:   bool,
    deserializable: bool,
    flattened:      bool,
}

impl Field {
    fn add_doc_attrs(&mut self, attrs: &[syn::Attribute]) {
        if let Some(val) = attrs::Doc::from_ast(attrs).comment {
            self.comment = quote! { Some(#val) };
        }
    }

    fn add_serde_attrs(&mut self, attrs: &[syn::Attribute]) -> Result<()> {
        let attrs::SerdeField {
            default: _,
            deserialize_with: _,
            flatten,
            rename,
            serialize_with: _,
            skip,
            skip_deserializing,
            skip_serializing,
            skip_serializing_if: _,
            with: _,
        } = attrs::SerdeField::from_ast(attrs)?;

        if let Some(val) = flatten {
            self.flattened = val;
        }

        if let Some(val) = rename {
            self.name = quote_spanned! { val.span() => #val };
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
        let attrs::DokuField {
            as_,
            example,
            flatten,
            rename,
            skip,
            tag,
        } = attrs::DokuField::from_ast(&attrs)?;

        if let Some(val) = as_ {
            let val = string_to_path(&val)?;
            self.ty = quote! { #val };
        }

        if let Some(val) = example {
            self.example = quote! { Some(#val) };
        }

        if let Some(val) = flatten {
            self.flattened = val;
        }

        if let Some(val) = rename {
            self.name = quote_spanned! { val.span() => #val };
        }

        if let Some(val) = skip {
            self.serializable = !val;
            self.deserializable = !val;
        }

        if let Some(val) = tag {
            self.tag = quote! { Some(#val) };
        }

        Ok(())
    }

    fn render(self, named: bool) -> TokenStream2 {
        let Self {
            name,
            ty,
            comment,
            example,
            tag,
            serializable,
            deserializable,
            flattened,
        } = self;

        if serializable || deserializable {
            let ty_def = quote! {
                let ty = <#ty as ::doku::TypeProvider>::ty();

                ::doku::Field {
                    ty: ::doku::Type {
                        comment: #comment,
                        example: #example.or(ty.example),
                        tag: #tag,
                        serializable: #serializable,
                        deserializable: #deserializable,
                        def: ty.def,
                    },

                    flattened: #flattened,
                }
            };

            if named {
                quote! {
                    (#name, { #ty_def }),
                }
            } else {
                quote! {
                    { #ty_def },
                }
            }
        } else {
            quote! {
                //
            }
        }
    }
}
