use difference::Changeset;
use std::fs;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! test_suite {
    // Given a list of test-cases (like `[ array/of_type/number ]`), builds a
    // list of test-functions in the following fashion:
    //
    // ```ignore
    // #[test]
    // fn array_of_type_number() {
    //     mod source {
    //         include!("json/array/of_type/number/ty.rs");
    //     }
    //
    //     json::test_case::<source::Ty>().run();
    // }
    //
    // #[test]
    // fn array_of_type_struct() {
    //     mod source {
    //         include!("json/array/of_type/struct/ty.rs");
    //     }
    //
    //     json::test_case::<source::Ty>().run();
    // }
    // ```
    //
    // This cannot be done at run-time, because we need the Rust compiler to
    // actually read & compile our `ty.rs` files (to check Doku's procedural
    // macros and perform an overall integration testing).
    ( $suite:ident, [ $( $( $name:ident )/+ )+ ] ) => {
        $(
            paste::item! {
                #[test]
                #[allow(dead_code)]
                #[allow(non_snake_case)]
                fn [< $( $name )_+ >] () {
                    mod source {
                        #[allow(unused_imports)]
                        use super::*;

                        include!(test_suite!(@file_name $suite $( $name )+));
                    }

                    let dir = Path::new("tests")
                        .join(test_suite!(@file_name $suite $( $name )+))
                        .parent()
                        .unwrap()
                        .to_owned();

                    test_case::<source::Ty>(&dir).run();
                }
            }
        )+
    };

    // Builds path to the `code.rs` of given test.
    //
    // E.g.:
    //
    // ```ignore
    // test_suite!(@file_name json array of_type number) == "json/array/of_type/number/code.rs"
    // ```
    ( @file_name $( $name:ident )+ ) => {
        concat!(
            $( stringify!($name), "/", )+
            "code.rs"
        )
    };
}

pub struct TestCase {
    /// Path to the `expected.xxx` file
    pub expected_path: PathBuf,

    /// Path to the `expected.xxx.new` file
    pub expected_new_path: PathBuf,

    /// Actual result of the test
    pub actual: String,
}

impl TestCase {
    pub fn run(self) {
        let Self {
            expected_path,
            expected_new_path,
            actual,
        } = self;

        if expected_new_path.exists() {
            remove(&expected_new_path);
        }

        let expected = if expected_path.exists() {
            read(&expected_path)
        } else {
            Default::default()
        };

        if expected != actual {
            write(&expected_new_path, &actual);

            let diff = Changeset::new(&expected, &actual, "\n");

            panic!(
                "Found differences between `{}` and `{}`:\n{}",
                expected_path.display(),
                expected_new_path.display(),
                diff,
            );
        }
    }
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|err| panic!("Couldn't read file `{}`: {}", path.display(), err))
}

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    fs::write(path, contents).unwrap_or_else(|err| panic!("Couldn't write file `{}`: {}", path.display(), err))
}

fn remove(path: impl AsRef<Path>) {
    let path = path.as_ref();
    fs::remove_file(path).unwrap_or_else(|err| panic!("Couldn't remove file `{}: {}`", path.display(), err))
}
