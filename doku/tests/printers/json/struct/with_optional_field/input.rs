// run: to_json()
// run: to_json_val()

#[derive(Serialize, Document)]
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

#[derive(Serialize, Document)]
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
