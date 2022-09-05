use crate::prelude::*;
use darling::FromMeta;

/// Models the `#[serde]` attribute for variants:
///
/// ```ignore
/// enum Foo {
///     #[serde(rename = "BAR")]
///     Bar,
/// }
/// ````
#[derive(Clone, Debug, Default, FromMeta)]
pub struct SerdeVariant {
    #[darling(default)]
    pub alias: Option<syn::LitStr>,

    #[darling(default)]
    pub deserialize_with: Option<syn::Meta>,

    #[darling(default)]
    pub other: Option<syn::Meta>,

    #[darling(default)]
    pub rename: Option<syn::LitStr>,

    #[darling(default)]
    pub serialize_with: Option<syn::Meta>,

    #[darling(default)]
    pub skip: Option<bool>,

    #[darling(default)]
    pub skip_deserializing: Option<bool>,

    #[darling(default)]
    pub skip_serializing: Option<bool>,

    #[darling(default)]
    pub with: Option<syn::LitStr>,
}

impl SerdeVariant {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Result<Self> {
        attrs::from_ast(attrs, "serde")
            .map(|attrs| attrs.fold(Self::default(), Self::merge))
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            alias: None,            // it's a no-op for us
            deserialize_with: None, // it's a no-op for us
            other: None,            // it's a no-op for us
            rename: other.rename.or(self.rename),
            serialize_with: None, // it's a no-op for us
            skip: other.skip.or(self.skip),
            skip_deserializing: other
                .skip_deserializing
                .or(self.skip_deserializing),
            skip_serializing: other.skip_serializing.or(self.skip_serializing),
            with: None, // it's a no-op for us
        }
    }
}
