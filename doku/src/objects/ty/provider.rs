use crate::*;

pub trait Provider {
    fn ty() -> ty::Type;
}

macro_rules! providers {
    (
        $(
            for $rust_ty:ty
            $(where ( $($tt:tt)+ ))?
            => $doku_ty:expr
        )+
    ) => {
        $(
            impl $(< $($tt)+ >)? ty::Provider for $rust_ty {
                fn ty() -> ty::Type {
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
