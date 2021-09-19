// run: test_json_ty

#[derive(Doku)]
#[doku(tag = "t", content = "c")]
pub enum Inner {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}

pub type Ty = Vec<Inner>;
