use super::*;

pub fn expand_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    rename_variants: RenameRule,
) -> Result<Vec<TokenStream2>> {
    variants
        .into_iter()
        .map(|v| expand_variant(v, rename_variants))
        .collect()
}
