// run: test_json_ty, test_json_val

#[derive(Doku)]
pub struct Ty {
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

#[derive(Doku)]
pub struct Foo {
    bar: Option<String>,
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            bar: Some("I'm Bar".to_string()),
        }
    }
}
