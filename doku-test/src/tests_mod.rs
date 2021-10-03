use crate::*;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct TestsMod<'a> {
    tests: BTreeMap<String, &'a Test>,
    children: BTreeMap<String, Self>,
}

impl<'a> TestsMod<'a> {
    pub fn build(tests: &'a [Test]) -> Self {
        tests.iter().fold(TestsMod::default(), |mut this, test| {
            this.add(test);
            this
        })
    }

    fn add(&mut self, test: &'a Test) {
        let mut this = self;
        let (test_dirs, test_name) = test.dirs_and_name();

        for test_dir in test_dirs {
            this = this.children.entry(test_dir).or_default();
        }

        this.tests.insert(test_name, test);
    }

    pub fn expand(&self) -> TokenStream {
        let tests = self.expand_tests();
        let children = self.expand_children();

        quote! {
            #(#tests)*
            #(#children)*
        }
    }

    fn expand_tests(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.tests.iter().map(|(name, test)| {
            let name = format_ident!("r#{}", name);
            let test = test.expand();

            quote! {
                mod #name {
                    use super::*;

                    #(#test)*
                }
            }
        })
    }

    fn expand_children(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.children.iter().map(|(name, children)| {
            let name = format_ident!("r#{}", name);
            let children = children.expand();

            quote! {
                mod #name {
                    use super::*;

                    #children
                }
            }
        })
    }
}
