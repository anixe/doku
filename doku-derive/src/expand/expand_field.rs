use super::*;
use crate::attrs::DokuMetas;
use std::iter::FromIterator;

pub fn expand_field(
    field: &syn::Field,
    named: bool,
    rename_fields: RenameRule,
) -> Result<TokenStream2> {
    let syn::Field {
        attrs, ident, ty, ..
    } = field;

    let ident = ident
        .as_ref()
        .map(|ident| rename_fields.apply_to_field(&ident.to_string()));

    let mut field = Field {
        name: quote! { #ident },
        ty: quote! { #ty },
        metas: quote! { Default::default() },
        comment: quote! { None },
        example: quote! { None },
        aliases: quote! { &[] },
        tag: quote! { None },
        serializable: true,
        deserializable: true,
        flattened: false,
    };

    field.add_doc_attrs(attrs);
    field.add_serde_attrs(&attrs)?;
    field.add_doku_attrs(&attrs)?;

    Ok(field.render(named))
}

struct Field {
    name: TokenStream2,
    ty: TokenStream2,
    comment: TokenStream2,
    aliases: TokenStream2,
    example: TokenStream2,
    metas: TokenStream2,
    tag: TokenStream2,
    serializable: bool,
    deserializable: bool,
    flattened: bool,
}

impl Field {
    fn add_doc_attrs(&mut self, attrs: &[syn::Attribute]) {
        if let Some(val) = attrs::Doc::from_ast(attrs).comment {
            self.comment = quote! { Some(#val) };
        }
    }

    fn add_serde_attrs(&mut self, attrs: &[syn::Attribute]) -> Result<()> {
        let attrs::SerdeField {
            alias,
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

        if !alias.is_empty() {
            self.aliases = quote! { &[#(#alias,)*] };
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
            examples,
            metas,
            literal_example,
            flatten,
            rename,
            skip,
            tag,
        } = attrs::DokuField::from_ast(&attrs)?;

        if let Some(val) = as_ {
            let val = string_to_path(&val)?;
            self.ty = quote! { #val };
        }

        if let Some(literal_example) = literal_example {
            self.example = quote! {
                Some(::doku::Example::Literal(#literal_example))
            };
        } else if !examples.is_empty() {
            self.example = quote! {
                Some(::doku::Example::from(&[#(#examples,)*][..]))
            };
        }

        if !metas.is_empty() {
            let metas = DokuMetas::from_iter(metas);

            let meta_keys = metas.metas.keys();
            let meta_values = metas.metas.values();

            self.metas = quote! {
                ::doku::Metas::default()
                #( .with(#meta_keys, #meta_values) )*
            };
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
            metas,
            tag,
            serializable,
            deserializable,
            flattened,
            aliases,
        } = self;

        if serializable || deserializable {
            let ty_kind = quote! {
                let ty = <#ty as ::doku::Document>::ty();

                ::doku::Field {
                    ty: ::doku::Type {
                        comment: #comment,
                        example: #example.or(ty.example),
                        metas: #metas,
                        tag: #tag,
                        serializable: #serializable,
                        deserializable: #deserializable,
                        kind: ty.kind,
                    },

                    flattened: #flattened,

                    aliases: #aliases,
                }
            };

            if named {
                quote! {
                    (#name, { #ty_kind }),
                }
            } else {
                quote! {
                    { #ty_kind },
                }
            }
        } else {
            quote! {
                //
            }
        }
    }
}
