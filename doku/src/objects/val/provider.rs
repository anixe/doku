// TODO(pwy) code-structure-wise, value providers should be paired with type
// providers

mod lang;
mod std;

use crate::*;

pub trait Provider {
    fn val(&self) -> Option<val::Value>;
}
