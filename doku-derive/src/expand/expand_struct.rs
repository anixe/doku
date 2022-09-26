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

    let generics = new_generics_with_where_clause(&input.generics)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::doku::Document for #ident #ty_generics #where_clause {
            fn ty() -> ::doku::Type {
                #ty
            }
        }
    })
}
