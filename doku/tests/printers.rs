#[allow(dead_code)]
#[path = "printers/array/mod.rs"]
mod array;

#[allow(dead_code)]
#[path = "printers/attribute/mod.rs"]
mod attribute;

#[allow(dead_code)]
#[path = "printers/enum/mod.rs"]
mod r#enum;

#[allow(dead_code)]
#[path = "printers/features/mod.rs"]
mod features;

#[allow(dead_code)]
#[path = "printers/formatting/mod.rs"]
mod formatting;

#[allow(dead_code)]
#[path = "printers/map/mod.rs"]
mod map;

#[allow(dead_code)]
#[path = "printers/metas/mod.rs"]
mod metas;

#[allow(dead_code)]
#[path = "printers/optional/mod.rs"]
mod optional;

#[allow(dead_code)]
#[path = "printers/struct/mod.rs"]
mod r#struct;

#[allow(dead_code)]
#[path = "printers/tuple/mod.rs"]
mod tuple;

// ---

mod prelude {
    #![allow(clippy::single_component_path_imports)]

    pub use super::*;
    pub use doku::Document;
    pub use serde::{Deserialize, Serialize};

    macro_rules! printer_test {
        (
            $(
                $file:literal => $assert_fn:ident $assert_args:tt
            ),+
            $(,)?
        ) => {
            #[test]
            fn test() {
                let mut expectations = Vec::new();

                $(
                    let expected = printer_test!(@assert $assert_fn $assert_args);
                    expectations.push(($file, expected));
                )+

                use std::path::{Path, PathBuf};

                let dir: PathBuf = Path::new(file!())
                    .parent()
                    .unwrap()
                    .iter()
                    .skip(1)
                    .collect();

                assert_dir(dir, expectations);
            }
        };

        (@assert to_json($ty:ty)) => {{
            doku::to_json::<$ty>()
        }};

        (@assert to_json_fmt($ty:ty, $fmt:tt)) => {{
            let fmt = serde_json::json!($fmt);
            let fmt = serde_json::from_value(fmt).expect("Given formatting is not valid");

            doku::to_json_fmt::<$ty>(&fmt)
        }};

        (@assert to_json_val($ty:ty)) => {{
            doku::to_json_val(&<$ty>::default())
        }};
    }

    pub(crate) use printer_test;
}

use difference::Changeset;
use std::fs;
use std::path::Path;

type FileName = &'static str;
type FileBody = String;
type DidAssertSucceed = bool;

pub fn assert_dir(
    dir: impl AsRef<Path>,
    expectations: Vec<(FileName, FileBody)>,
) {
    let dir = dir.as_ref();
    let mut all_asserts_succeeded = true;

    for (file, expected) in expectations {
        all_asserts_succeeded &= assert(dir, file, expected);
    }

    if !all_asserts_succeeded {
        panic!("Some assertions failed");
    }
}

fn assert(dir: &Path, file: FileName, expected: FileBody) -> DidAssertSucceed {
    let path = dir.join(file);

    let path_new = path.with_extension(format!(
        "{}.new",
        path.extension().unwrap().to_string_lossy()
    ));

    if path_new.exists() {
        fs::remove_file(&path_new).unwrap_or_else(|err| {
            panic!(
                "Couldn't remove new-fixture `{}`: {}",
                path_new.display(),
                err
            )
        })
    }

    let actual = if path.exists() {
        fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!("Couldn't read fixture `{}`: {}", path.display(), err)
        })
    } else {
        Default::default()
    };

    if actual == expected {
        true
    } else {
        fs::write(&path_new, &expected).unwrap_or_else(|err| {
            panic!("Couldn't write fixture `{}`: {}", path_new.display(), err)
        });

        eprintln!(
            "\nFound differences between `{}` and `{}`:\n{}",
            path.display(),
            path_new.display(),
            Changeset::new(&actual, &expected, "\n"),
        );

        false
    }
}
