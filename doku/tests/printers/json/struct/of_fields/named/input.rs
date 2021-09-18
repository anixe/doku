// run: to_json()
// run: to_json_val()

#[derive(Serialize, Document)]
pub struct Ty {
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
