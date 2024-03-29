mod attrs;
mod error;
mod expand;
mod utils;

mod prelude {
    pub(crate) use super::{
        attrs::{self, RenameRule},
        error::*,
        utils::*,
    };
    pub use proc_macro::TokenStream;
    pub use proc_macro2::{Span, TokenStream as TokenStream2};
    pub use quote::{quote, quote_spanned};
    pub use syn::spanned::Spanned;

    pub type Result<T, E = Error> = std::result::Result<T, E>;
}

use self::prelude::*;
use syn::parse_macro_input;

#[proc_macro_derive(Document, attributes(doku))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match expand::expand(&input) {
        Ok(stream) => stream,

        Err(err) => {
            let err = err.compile();
            let ident = input.ident;

            // We're emitting a dummy impl to avoid a potential error-cascade
            // when something else already expects this type to be doku-fied
            quote! {
                #err

                impl ::doku::Document for #ident {
                    fn ty() -> ::doku::Type {
                        unreachable!()
                    }
                }
            }
        }
    }
    .into()
}
