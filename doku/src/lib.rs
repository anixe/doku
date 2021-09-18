#![feature(crate_visibility_modifier)]

mod objects;
mod printers;

pub mod prelude {
    pub use doku_derive::*;
}

pub use self::{objects::*, printers::*};

pub fn to_json<Ty: TypeProvider>() -> String {
    JsonPrinter::new().print(&Ty::ty())
}

pub fn to_json_val<Ty: TypeProvider>(_: &Ty) -> String {
    to_json::<Ty>()
}
