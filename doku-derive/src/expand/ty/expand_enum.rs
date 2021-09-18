use super::*;

pub fn expand_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> Result<TokenStream2> {
    let syn::DeriveInput { ident, .. } = input;
    let doku = attrs::DokuContainer::from_ast(&input.attrs)?;
    let serde = attrs::SerdeContainer::from_ast(&input.attrs)?;

    let ty_def = {
        let untagged = doku.untagged.or(serde.untagged);
        let content = doku.content.as_ref().or_else(|| serde.content.as_ref());
        let tag = doku.tag.as_ref().or_else(|| serde.tag.as_ref());

        let tag = if untagged.unwrap_or(false) {
            quote! {
                ::doku::ty::Tag::None
            }
        } else {
            match (content, tag) {
                (Some(content), Some(tag)) => quote! {
                    ::doku::ty::Tag::Adjacent {
                        content: #content,
                        tag: #tag,
                    }
                },

                (Some(_), None) => {
                    // This is illegal, but either rustc or serde should've
                    // issued an appropriate error message by now, so there's no
                    // need for us to linger too

                    quote! {
                        ::doku::ty::Tag::External
                    }
                }

                (None, Some(tag)) => quote! {
                    ::doku::ty::Tag::Internal {
                        tag: #tag,
                    }
                },

                (None, None) => quote! {
                    ::doku::ty::Tag::External
                },
            }
        };

        let variants = expand_variants(&data.variants)?;

        quote! {
            ::doku::ty::Def::Enum {
                tag: #tag,
                variants: vec![ #(#variants)* ],
            }
        }
    };

    let ty = {
        let mut ty = quote! {
            ::doku::ty::Type::from_def( #ty_def )
        };

        if let Some(wrap) = doku.wrap {
            ty = expand_wrap(wrap, ty);
        }

        ty
    };

    Ok(quote! {
        impl ::doku::ty::Provider for #ident {
            fn ty() -> ::doku::ty::Type {
                #ty
            }
        }
    })
}
