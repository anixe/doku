use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[doku]` attribute for containers:
///
/// ```ignore
/// #[doku(transparent)]
/// struct Foo {
///     field: Bar,
/// }
/// ````
#[derive(Clone, Debug, Default, FromMeta)]
pub struct DokuContainer {
    #[darling(default)]
    pub content: Option<syn::LitStr>,

    #[darling(default)]
    pub rename_all: Option<RenameRule>,

    #[darling(default)]
    pub tag: Option<syn::LitStr>,

    #[darling(default)]
    pub transparent: Option<bool>,

    #[darling(default)]
    pub untagged: Option<bool>,
}

impl DokuContainer {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "doku")
            .map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    fn merge(self, other: Self) -> Self {
        Self {
            content: other.content.or(self.content),
            rename_all: other.rename_all.or(self.rename_all),
            tag: other.tag.or(self.tag),
            transparent: other.transparent.or(self.transparent),
            untagged: other.untagged.or(self.untagged),
        }
    }
}
