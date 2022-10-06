use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(rename_all = "lowercase")]
enum Ty {
    #[serde(rename_all = "kebab-case")]
    Foo {
        /// This should be kebab-case
        a_field_with_many_words: String,

        /// This should be renamed to foo
        #[serde(rename = "foo")]
        another_field_with_many_words: String,
    },
}

printer_test! {
    "output.json" => to_json(Ty),
}
