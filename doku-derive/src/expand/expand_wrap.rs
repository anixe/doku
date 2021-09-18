use super::*;

pub fn expand_wrap(
    field_name: syn::LitStr,
    field_type: TokenStream2,
) -> TokenStream2 {
    quote! {
        let field = ::doku::Field {
            ty: #field_type,
            flattened: false,
        };

        ::doku::Type::from(::doku::TypeKind::Struct {
            fields: ::doku::Fields::Named {
                fields: vec![ (#field_name, field) ],
            },

            transparent: false,
        })
    }
}
