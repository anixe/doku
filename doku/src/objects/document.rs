use crate::*;

/// A type that's understandable by Doku.
///
/// Usually you'll get this by adding `#[derive(Document)]` to your type:
///
/// ```
/// use doku::Document;
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

mod lang;
mod std;

#[cfg(feature = "chrono-04")]
mod chrono_04;

#[cfg(feature = "url-2")]
mod url_2;
