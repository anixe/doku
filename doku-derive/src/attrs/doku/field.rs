use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[doku]` attribute for fields:
///
/// ```ignore
/// struct Foo {
///     #[doku(as = "Zar")]
///     field: Bar,
/// }
/// ````
#[derive(Clone, Debug, Default, FromMeta)]
pub struct DokuField {
    #[darling(default, rename = "as")]
    pub as_: Option<syn::LitStr>,

    #[darling(default, rename = "example", multiple)]
    pub examples: Vec<syn::LitStr>,

    #[darling(default)]
    pub flatten: Option<bool>,

    #[darling(default)]
    pub rename: Option<syn::LitStr>,

    #[darling(default)]
    pub skip: Option<bool>,

    #[darling(default)]
    pub tag: Option<syn::LitStr>,
}

impl DokuField {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "doku")
            .map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    fn merge(self, other: Self) -> Self {
        let examples =
            self.examples.into_iter().chain(other.examples).collect();

        Self {
            as_: other.as_.or(self.as_),
            examples,
            flatten: other.flatten.or(self.flatten),
            rename: other.rename.or(self.rename),
            skip: other.skip.or(self.skip),
            tag: other.tag.or(self.tag),
        }
    }
}
