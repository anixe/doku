mod doc;
mod doku;
mod serde;

pub use self::{doc::*, doku::*, serde::*};

use crate::prelude::*;
use darling::FromMeta;

pub(self) fn from_ast<T>(
    attrs: &[syn::Attribute],
    name: &str,
) -> Result<impl Iterator<Item = T>>
where
    T: FromMeta,
{
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| path_to_string(&attr.path) == name)
        .map(|attr| {
            let meta = attr.parse_meta().emit_err()?;
            let this = T::from_meta(&meta).emit_err()?;
            Ok(this)
        })
        .collect::<Result<_>>()?;

    Ok(attrs.into_iter())
}
