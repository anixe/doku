#[derive(Doku)]
pub enum Inner {
    /// This is `Foo`
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
}

pub type Ty = Vec<Inner>;
