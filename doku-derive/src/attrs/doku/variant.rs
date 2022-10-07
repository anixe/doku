use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[doku]` attribute for variants:
///
/// ```ignore
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
    pub rename_all: Option<RenameRule>,

    #[darling(default)]
    pub skip: Option<bool>,
}

impl DokuVariant {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "doku")
            .map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    fn merge(self, other: Self) -> Self {
        Self {
            rename: other.rename.or(self.rename),
            rename_all: other.rename_all.or(self.rename_all),
            skip: other.skip.or(self.skip),
        }
    }
}
