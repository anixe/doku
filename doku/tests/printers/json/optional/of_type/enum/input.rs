// run: to_json()

#[derive(Document)]
pub enum Enum {
    Foo,
    Bar,
}

pub type Ty = Option<Enum>;
