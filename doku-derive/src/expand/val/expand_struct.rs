use super::*;

pub fn expand_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream2 {
    let syn::DeriveInput { ident, .. } = input;
    let fields = expand_fields(&data.fields);

    quote! {
        impl ::doku::val::Provider for #ident {
            fn val(&self) -> Option<::doku::val::Value> {
                Some(::doku::val::Value::Struct {
                    fields: #fields,
                })
            }
        }
    }
}
