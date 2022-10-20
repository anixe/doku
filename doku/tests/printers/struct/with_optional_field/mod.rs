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
    "output.without-key-quotes.ty.json" => to_json_without_key_quotes(Ty),
    "output.without-key-quotes.val.json" => to_json_val_without_key_quotes(Ty),
    "output.ty.toml" => to_toml(Ty),
    "output.val.toml" => to_toml_val(Ty),
}
