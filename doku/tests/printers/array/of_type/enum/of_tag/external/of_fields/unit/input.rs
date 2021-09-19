// run: test_json_ty

#[derive(Doku)]
pub enum Inner {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}

pub type Ty = Vec<Inner>;
