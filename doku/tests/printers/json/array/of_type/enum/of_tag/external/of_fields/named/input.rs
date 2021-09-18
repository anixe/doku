// run: to_json()

#[derive(Document)]
pub enum Inner {
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

pub type Ty = Vec<Inner>;
