use darling::{Error, FromMeta, Result};
use std::{collections::BTreeMap, iter::FromIterator};

const ERROR_EXPECTED_KV_LITERAL: &str =
    "Expected a key-value string literal such as: `#[doku(meta(\"key = value\"))]`";

#[derive(Clone, Debug, Default)]
pub struct DokuMetas {
    pub metas: BTreeMap<String, String>,
}

impl FromMeta for DokuMetas {
    fn from_list(items: &[syn::NestedMeta]) -> Result<Self> {
        let metas = items
            .iter()
            .map(|item| match item {
                syn::NestedMeta::Lit(syn::Lit::Str(lit)) => {
                    Ok((lit, lit.value()))
                }

                _ => {
                    Err(Error::custom(ERROR_EXPECTED_KV_LITERAL)
                        .with_span(item))
                }
            })
            .map(|item| {
                item.and_then(|(lit, lit_value)| {
                    let mut kv = lit_value.splitn(2, '=');

                    let key = kv.next().ok_or_else(|| {
                        Error::custom(ERROR_EXPECTED_KV_LITERAL).with_span(lit)
                    })?;

                    let value = kv.next().ok_or_else(|| {
                        Error::custom(ERROR_EXPECTED_KV_LITERAL).with_span(lit)
                    })?;

                    Ok((key.trim().to_owned(), value.trim().to_owned()))
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self { metas })
    }
}

impl FromIterator<DokuMetas> for DokuMetas {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = DokuMetas>,
    {
        Self {
            metas: iter.into_iter().flat_map(|metas| metas.metas).collect(),
        }
    }
}
