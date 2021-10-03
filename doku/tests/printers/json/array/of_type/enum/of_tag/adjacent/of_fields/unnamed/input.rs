// run: to_json()

#[derive(Document)]
#[doku(tag = "t", content = "c")]
pub enum Inner {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}

pub type Ty = Vec<Inner>;
