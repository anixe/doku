use crate::*;

pub trait TypeProvider {
    fn ty() -> Type;
}

macro_rules! type_providers {
    (
        $(
            for $rust_ty:ty
            $(where ( $($tt:tt)+ ))?
            => $doku_ty:expr
        )+
    ) => {
        $(
            impl $(< $($tt)+ >)? TypeProvider for $rust_ty {
                fn ty() -> Type {
                    $doku_ty.into()
                }
            }
        )+
    };
}

#[cfg(feature = "chrono-04")]
mod chrono;

mod lang;
mod std;
