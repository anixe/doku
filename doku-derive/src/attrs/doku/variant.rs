use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[doku]` attribute for variants:
///
/// ```rust,ignore
/// enum Foo {
///     #[doku(rename = "BAR")]
///     Bar,
/// }
/// ````
#[derive(Clone, Debug, Default, FromMeta)]
pub struct DokuVariant {
    #[darling(default)]
    pub rename: Option<syn::LitStr>,

    #[darling(default)]
    pub skip: Option<bool>,
}

impl DokuVariant {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "doku").map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    fn merge(self, other: Self) -> Self {
        Self {
            rename: other.rename.or(self.rename),
            skip:   other.skip.or(self.skip),
        }
    }
}
