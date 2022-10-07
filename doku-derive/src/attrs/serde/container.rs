use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[serde]` attribute for containers:
///
/// ```ignore
/// #[serde(transparent)]
/// struct Foo {
///     field: Bar,
/// }
/// ````
#[derive(Clone, Debug, Default, FromMeta)]
pub struct SerdeContainer {
    #[darling(default)]
    pub content: Option<syn::LitStr>,

    #[darling(default)]
    #[darling(rename = "crate")]
    pub krate: Option<syn::Meta>,

    #[darling(default)]
    pub default: Option<syn::Meta>,

    #[darling(default)]
    pub deny_unknown_fields: Option<syn::Meta>,

    #[darling(default)]
    pub rename_all: Option<RenameRule>,

    #[darling(default)]
    pub tag: Option<syn::LitStr>,

    #[darling(default)]
    pub transparent: Option<bool>,

    #[darling(default)]
    pub untagged: Option<bool>,
}

impl SerdeContainer {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "serde")
            .map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            content: other.content.or(self.content),
            krate: None,               // it's a no-op for us
            default: None,             // it's a no-op for us
            deny_unknown_fields: None, // it's a no-op for us
            rename_all: other.rename_all.or(self.rename_all),
            tag: other.tag.or(self.tag),
            transparent: other.transparent.or(self.transparent),
            untagged: other.untagged.or(self.untagged),
        }
    }
}
