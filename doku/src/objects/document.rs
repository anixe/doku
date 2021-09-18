use crate::*;

/// A type that's understandable by Doku.
///
/// Usually you'll get this by adding `#[derive(Document)]` to your type:
///
/// ```
/// use doku::prelude::*;
///
/// #[derive(Document)]
/// struct Foo;
/// ```
pub trait Document {
    fn ty() -> Type;
}

macro_rules! document {
    (
        $(
            for $ty:ty
            $(where ($($impl:tt)+) $({ $($where:tt)+ })?)?
            => $expr:expr;
        )+
    ) => {
        $(
            impl $(< $($impl)+ >)? Document for $ty
            $( $( where $($where)+ )? )?
            {
                fn ty() -> Type {
                    $expr
                }
            }
        )+
    };
}

#[cfg(feature = "chrono-04")]
mod chrono;
mod lang;
mod std;
