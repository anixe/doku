use super::*;

pub fn expand_fields(
    fields: &syn::Fields,
    rename_fields: RenameRule,
) -> Result<TokenStream2> {
    match fields {
        syn::Fields::Named(inner) => expand_named_fields(inner, rename_fields),
        syn::Fields::Unnamed(inner) => expand_unnamed_fields(inner),
        syn::Fields::Unit => Ok(expand_unit_fields()),
    }
}

fn expand_named_fields(
    fields: &syn::FieldsNamed,
    rename_fields: RenameRule,
) -> Result<TokenStream2> {
    let fields: Vec<_> = fields
        .named
        .iter()
        .map(|field| expand_field(field, true, rename_fields))
        .collect::<Result<_>>()?;

    Ok(quote! {
        ::doku::Fields::Named {
            fields: vec![ #(#fields)* ],
        }
    })
}

fn expand_unnamed_fields(fields: &syn::FieldsUnnamed) -> Result<TokenStream2> {
    let fields: Vec<_> = fields
        .unnamed
        .iter()
        .map(|field| expand_field(field, false, RenameRule::None))
        .collect::<Result<_>>()?;

    Ok(quote! {
        ::doku::Fields::Unnamed {
            fields: vec![ #(#fields)* ],
        }
    })
}

fn expand_unit_fields() -> TokenStream2 {
    quote! {
        ::doku::Fields::Unit
    }
}
