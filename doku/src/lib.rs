#![feature(crate_visibility_modifier)]

mod objects;
mod printers;

pub mod prelude {
    pub use doku_derive::*;
}

pub use self::{objects::*, printers::*};

pub fn to_json<T: ty::Provider>() -> String {
    JsonPrinter::new().print(&T::ty())
}

pub fn to_json_val<T: ty::Provider + val::Provider>(val: &T) -> String {
    JsonPrinter::new().with_value(val.val()).print(&T::ty())
}
