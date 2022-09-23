use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    /// Some comment
    foo: Option<Foo>,
}

impl Default for Ty {
    fn default() -> Self {
        Self {
            foo: Some(Default::default()),
        }
    }
}

#[derive(Serialize, Document)]
struct Foo {
    bar: Option<String>,
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            bar: Some("I'm Bar".to_string()),
        }
    }
}

printer_test! {
    "output.ty.json" => to_json(Ty),
    "output.val.json" => to_json_val(Ty),
    "output.without_key_quotes.ty.json" => to_json_without_key_quotes(Ty),
    "output.without_quotes.val.json" => to_json_val_without_key_quotes(Ty),
}
