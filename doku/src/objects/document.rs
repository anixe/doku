use crate::*;

/// Type that can be pretty-printed by Doku.
///
/// Usually you'll get this by adding `#[derive(Document)]` to your type:
///
/// ```
/// use doku::Document;
///
/// #[derive(Document)]
/// struct Foo;
/// ```
///
/// ... but implementing it manually will be required if you're using a custom
/// serializer / deserializer.
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
                    $expr.into()
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
