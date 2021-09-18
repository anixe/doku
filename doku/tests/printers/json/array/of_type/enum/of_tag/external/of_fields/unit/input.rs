// run: to_json()

#[derive(Document)]
pub enum Inner {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}

pub type Ty = Vec<Inner>;
