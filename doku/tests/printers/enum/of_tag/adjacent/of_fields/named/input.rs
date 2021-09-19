// run: test_json_ty

#[derive(Doku)]
#[doku(tag = "t", content = "c")]
pub enum Ty {
    /// This is `Foo`
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}
