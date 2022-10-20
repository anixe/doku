use crate::prelude::*;

#[derive(Serialize, Document)]
struct Ty {
    /// Some comment
    foo: String,

    /// Some comment
    bar: usize,
}

impl Default for Ty {
    fn default() -> Self {
        Self {
            foo: "I'm Foo".to_string(),
            bar: 4096,
        }
    }
}

printer_test! {
    "output.ty.json" => to_json(Ty),
    "output.val.json" => to_json_val(Ty),
    "output.without-comma.ty.json" => to_json_without_comma(Ty),
    "output.without-comma.val.json" => to_json_val_without_comma(Ty),
    "output.without-key-quotes.ty.json" => to_json_without_key_quotes(Ty),
    "output.without-key-quotes.val.json" => to_json_val_without_key_quotes(Ty),
    "output.ty.toml" => to_toml(Ty),
    "output.val.toml" => to_toml_val(Ty),
}
