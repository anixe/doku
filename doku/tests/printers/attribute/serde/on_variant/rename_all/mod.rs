use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(rename_all = "lowercase")]
enum Ty {
    /// This should be lowercase
    #[serde(rename_all = "kebab-case")]
    OneVariant {
        /// This should be kebab-case
        a_field_with_many_words: String,

        /// This should be renamed to foo
        #[serde(rename = "foo")]
        another_field_with_many_words: String,
    },

    /// This should also be lowercase
    AnotherVariant {
        /// This should remain unchanged
        a_field_that_remains_unchanged: String
    }
}

printer_test! {
    "output.json" => to_json(Ty),
}
