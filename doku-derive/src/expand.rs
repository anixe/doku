mod ty;
mod val;

use crate::prelude::*;

pub fn expand(input: &syn::DeriveInput) -> Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        emit_error!(
            input.generics.params.span(),
            "Generic types are not supported yet; please `impl doku::ty::Provider for ...` by hand"
        );

        return Err(());
    }

    match &input.data {
        syn::Data::Struct(data) => expand_struct(input, data),
        syn::Data::Enum(data) => expand_enum(input, data),
        syn::Data::Union(_) => {
            emit_error!(input.ident.span(), "Unions are not supported yet");
            Err(())
        }
    }
}

fn expand_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> Result<TokenStream2> {
    let ty = ty::expand_struct(input, data)?;
    let val = val::expand_struct(input, data);

    Ok(quote! {
        #ty
        #val
    })
}

fn expand_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> Result<TokenStream2> {
    let ty = ty::expand_enum(input, data)?;
    let val = val::expand_enum(input, data);

    Ok(quote! {
        #ty
        #val
    })
}
