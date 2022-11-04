mod expand_enum;
mod expand_field;
mod expand_fields;
mod expand_struct;
mod expand_variant;
mod expand_variants;
mod utils;

use self::{
    expand_enum::expand_enum, expand_field::expand_field,
    expand_fields::expand_fields, expand_struct::expand_struct,
    expand_variant::expand_variant, expand_variants::expand_variants, utils::*,
};
use crate::prelude::*;

pub fn expand(input: &syn::DeriveInput) -> Result<TokenStream2> {
    match &input.data {
        syn::Data::Struct(data) => expand_struct(input, data),
        syn::Data::Enum(data) => expand_enum(input, data),
        syn::Data::Union(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "unions are not supported yet",
        )
        .into()),
    }
}
