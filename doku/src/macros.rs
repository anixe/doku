#[doc(hidden)]
#[macro_export]
macro_rules! assert_doc {
    ($expected:expr, $actual:expr) => {{
        use ::difference::Changeset;

        let expected = indoc::indoc! { $expected }.trim();
        let actual = $actual;
        let actual = actual.trim();

        if expected != actual {
            panic!(
                "\n<expected>\n{}\n</expected>\n\n<actual>\n{}\n</actual>\n\n<diff>\n{}\n</diff>",
                expected,
                actual,
                Changeset::new(&actual, &expected, "\n").to_string().trim(),
            );
        }
    }};
}

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
