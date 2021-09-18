use super::*;

pub fn expand_wrap(field_name: syn::LitStr, field_type: TokenStream2) -> TokenStream2 {
    quote! {
        let field_type_def = ::doku::ty::Field {
            ty: #field_type,
            flattened: false,
        };

        ::doku::ty::Type::from_def(::doku::ty::Def::Struct {
            fields: ::doku::ty::Fields::Named {
                fields: vec![ (#field_name, field_type_def) ]
            },

            transparent: false,
        })
    }
}
