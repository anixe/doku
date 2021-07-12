#[derive(Doku)]
pub struct Inner {
    /// Some comment
    foo: String,

    /// Some comment
    bar: usize,
}

pub type Ty = Vec<Inner>;
