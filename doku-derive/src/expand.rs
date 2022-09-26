mod expand_enum;
mod expand_field;
mod expand_fields;
mod expand_struct;
mod expand_variant;
mod expand_variants;
mod expand_wrap;

use self::{
    expand_enum::expand_enum, expand_field::expand_field,
    expand_fields::expand_fields, expand_struct::expand_struct,
    expand_variant::expand_variant, expand_variants::expand_variants,
    expand_wrap::expand_wrap,
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

fn new_generics_with_where_clause(
    generics: &syn::Generics,
) -> Result<syn::Generics> {
    let mut new_generics = generics.to_owned();
    let where_clause = new_generics.make_where_clause();
    for generic in &generics.params {
        match generic {
            syn::GenericParam::Const(_) => (),
            syn::GenericParam::Lifetime(_) => (),
            syn::GenericParam::Type(t) => {
                let predicate: syn::WherePredicate =
                    syn::parse2(quote! { #t: ::doku::Document }).unwrap();
                where_clause.predicates.push(predicate);
            }
        };
    }
    Ok(new_generics)
}
