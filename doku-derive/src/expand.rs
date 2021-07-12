mod expand_enum;
mod expand_field;
mod expand_fields;
mod expand_struct;
mod expand_variant;
mod expand_variants;
mod expand_wrap;

use self::{
    expand_enum::*,
    expand_field::*,
    expand_fields::*,
    expand_struct::*,
    expand_variant::*,
    expand_variants::*,
    expand_wrap::*,
};

use crate::prelude::*;

pub fn expand(input: &syn::DeriveInput) -> Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        emit_error!(
            input.generics.params.span(),
            "Generic types are not supported yet; please `impl doku::TypeProvider for ...` by hand"
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
