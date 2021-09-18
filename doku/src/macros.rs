#[macro_export]
macro_rules! assert_doc {
    ($expected:expr, $actual:expr) => {
        assert_eq!(indoc::indoc! { $expected }.trim(), $actual.trim());
    };
}

#[macro_export]
macro_rules! swrite {
    ($target:expr, if $expr:expr, $($tt:tt)+) => {
        if $expr {
           swrite!($target, $($tt)+);
        }
    };

    ($target:expr, for $expr:expr, $($tt:tt)+) => {
        for _ in $expr {
           swrite!($target, $($tt)+);
        }
    };

    ($target:expr, $($tt:tt)+) => {
        let _ = write!($target, $($tt)+);
    };
}
