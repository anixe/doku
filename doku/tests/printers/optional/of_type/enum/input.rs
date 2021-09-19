// run: test_json_ty

#[derive(Doku)]
pub enum Enum {
    Foo,
    Bar,
}

pub type Ty = Option<Enum>;
