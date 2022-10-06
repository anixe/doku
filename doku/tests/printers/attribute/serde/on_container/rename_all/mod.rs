use crate::prelude::*;

#[derive(Serialize, Document)]
#[serde(rename_all = "kebab-case")]
struct Ty {
    /// This should be kebab-case
    a_field_with_many_words: String,

    /// This should be renamed to foo
    #[serde(rename = "foo")]
    another_field_with_many_words: Enum,
}

#[derive(Serialize, Document)]
#[serde(rename_all = "lowercase")]
enum Enum {
    /// This should be lowercase
    AVariantWithManyWords,

    /// This should be renamed to foo
    #[serde(rename = "foo")]
    AnotherVariantWithManyWords,
}

printer_test! {
    "output.json" => to_json(Ty),
}
