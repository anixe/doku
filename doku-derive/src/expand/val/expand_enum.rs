use super::*;

// TODO
pub fn expand_enum(input: &syn::DeriveInput, _: &syn::DataEnum) -> TokenStream2 {
    let syn::DeriveInput { ident, .. } = input;

    quote! {
        impl ::doku::val::Provider for #ident {
            fn val(&self) -> Option<::doku::val::Value> {
                None
            }
        }
    }
}
