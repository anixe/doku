// run: test_json_ty

#[derive(Doku)]
#[doku(tag = "t", content = "c")]
pub enum Inner {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}

pub type Ty = Vec<Inner>;
