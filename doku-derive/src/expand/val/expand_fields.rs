use super::*;

pub fn expand_fields(fields: &syn::Fields) -> TokenStream2 {
    match fields {
        syn::Fields::Named(inner) => expand_named_fields(inner),
        syn::Fields::Unnamed(inner) => expand_unnamed_fields(inner),
        syn::Fields::Unit => expand_unit_fields(),
    }
}

fn expand_named_fields(fields: &syn::FieldsNamed) -> TokenStream2 {
    let fields: Vec<_> = fields
        .named
        .iter()
        .map(|field| {
            let syn::Field { ident, .. } = field;
            let name = quote! { stringify!(#ident) };
            let val = quote! { ::doku::val::Provider::val(&self.#ident) };

            quote! {
                (#name, #val),
            }
        })
        .collect();

    quote! {
        ::doku::val::Fields::Named {
            fields: vec![ #(#fields)* ]
                .into_iter()
                .flat_map(|(name, value)| Some((name, value?)))
                .collect(),
        }
    }
}

fn expand_unnamed_fields(fields: &syn::FieldsUnnamed) -> TokenStream2 {
    let fields: Vec<_> = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(id, _)| {
            let id = syn::Index::from(id);

            quote! {
                ::doku::val::Provider::val(&self.#id),
            }
        })
        .collect();

    quote! {
        ::doku::val::Fields::Unnamed {
            fields: vec![ #(#fields)* ]
                .into_iter()
                .flatten()
                .collect(),
        }
    }
}

fn expand_unit_fields() -> TokenStream2 {
    quote! {
        ::doku::val::Fields::Unit
    }
}
