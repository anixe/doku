mod test;
mod tests;

use self::test::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::{fmt::Write, path::Component};

#[proc_macro]
pub fn printers(_: TokenStream) -> TokenStream {
    let tests = tests::discover("doku/tests/printers");

    let tests = tests.iter().flat_map(|test| {
        let name = test
            .dir()
            .components()
            .skip(2)
            .fold(String::default(), |mut name, component| {
                if let Component::Normal(dir) = component {
                    write!(&mut name, "{}_", dir.to_string_lossy()).unwrap();
                }

                name
            });

        let path = test.dir().strip_prefix("doku/tests/").unwrap();
        let dir = path.display().to_string();
        let input = path.join("input.rs").display().to_string();

        test.run().iter().map(move |run| {
            let name = format_ident!("{}_{}", name, run);
            let run = format_ident!("{}", run);

            quote! {
                #[test]
                fn #name() {
                    mod source {
                        #[allow(unused_imports)]
                        use super::*;

                        include!(#input);
                    }

                    #run::<source::Ty>(Path::new(#dir));
                }
            }
        })
    });

    (quote! {
        #(#tests)*
    })
    .into()
}
