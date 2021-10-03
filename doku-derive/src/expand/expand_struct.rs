use super::*;

pub fn expand_struct(
    input: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> Result<TokenStream2> {
    let syn::DeriveInput { ident, .. } = input;
    let doku = attrs::DokuContainer::from_ast(&input.attrs)?;
    let serde = attrs::SerdeContainer::from_ast(&input.attrs)?;

    let ty = {
        let fields = expand_fields(&data.fields)?;

        let transparent = {
            let transparent =
                doku.transparent.or(serde.transparent).unwrap_or(false);
            quote! { #transparent }
        };

        let mut ty = quote! {
            ::doku::Type::from(::doku::TypeKind::Struct {
                fields: #fields,
                transparent: #transparent,
            })
        };

        if let Some(wrap) = doku.wrap {
            ty = expand_wrap(wrap, ty);
        }

        ty
    };

    Ok(quote! {
        impl ::doku::Document for #ident {
            fn ty() -> ::doku::Type {
                #ty
            }
        }
    })
}
