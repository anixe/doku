use crate::prelude::*;

pub fn new_generics_with_where_clause(
    generics: &syn::Generics,
) -> Result<syn::Generics> {
    let mut new_generics = generics.to_owned();
    let where_clause = new_generics.make_where_clause();
    for generic in &generics.params {
        match generic {
            syn::GenericParam::Const(_) => (),
            syn::GenericParam::Lifetime(_) => (),
            syn::GenericParam::Type(t) => {
                let t = &t.ident;
                let predicate: syn::WherePredicate =
                    syn::parse2(quote! { #t: ::doku::Document })?;
                where_clause.predicates.push(predicate);
            }
        };
    }
    Ok(new_generics)
}
