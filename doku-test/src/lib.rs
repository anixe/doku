mod test;
mod tests;
mod tests_mod;

use self::{test::*, tests::*, tests_mod::*};
use anyhow::{bail, Context as _, Result};
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[proc_macro]
pub fn printers(_: TokenStream1) -> TokenStream1 {
    let tests = match Tests::discover("doku/tests/printers") {
        Ok(tests) => tests,
        Err(err) => {
            panic!("\n{:?}", err);
        }
    };

    let tests = tests.expand();

    (quote! {
        #tests
    })
    .into()
}
