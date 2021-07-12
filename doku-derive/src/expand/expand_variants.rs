use super::*;

pub fn expand_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> Result<Vec<TokenStream2>> {
    variants.into_iter().map(|variant| expand_variant(variant)).collect()
}
