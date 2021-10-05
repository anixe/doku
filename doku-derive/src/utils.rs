use crate::prelude::*;
use syn::{LitStr, Path};

pub fn path_to_string(path: &Path) -> String {
    quote!(#path).to_string()
}

/// Transforms a `syn::LitStr` representing a path (e.g.
/// `"std::collections::HashMap"`) into an actual instance of `syn::Path` that
/// can be then fed into the `quote!` macro.
///
/// # Abstract
///
/// While working with attributes, we internally rely on `syn::Meta` which
/// supports maps with literal values only; for instance, it understands
/// `#[foo(bar = "somewhere::SomeStruct")]`, but fails on
/// `#[foo(bar = somewhere::SomeStruct)]` (or at least returns data in a
/// somewhat inconvenient format).
///
/// On the other hand, we can't use `syn::LitStr` inside `quote!` in a type
/// position (e.g. `quote! { type x = #str }`), because `quote!` always
/// automatically escapes all strings (e.g. into
/// `type x = "somewhere::SomeString"`).
///
/// Thus this function is responsible for transforming `syn::LitStr` into a
/// `syn::Path` it should've been from the beginning.
pub fn string_to_path(str: &LitStr) -> Result<Path> {
    let mut path: Path = syn::parse_str(&str.value())?;

    // Because we're using `syn::parse_str()`, we lose all information about the
    // original value's span - this means that when someone makes a typo,
    // compiler would complain underlining the entire `#[doku(...)]` annotation,
    // instead of just the type name - thus code below is responsible for
    // re-wiring the spans.
    //
    // TODO Overall, code below is kind of a hack that'll work in most of the
    //      cases, with the exception being path arguments (e.g.
    //      `Vec<SomeType>`) - we don't modify their spans, because it'd
    //      require a bit more work.

    for segment in path.segments.iter_mut() {
        segment.ident.set_span(str.span());
    }

    Ok(path)
}
