use difference::Changeset;
use doku::prelude::*;
use serde::*;
use std::fs;
use std::path::Path;

mod tests {
    #![allow(dead_code)]

    use super::*;
    use serde_json::json;

    doku_test::printers!();
}

fn to_json<T>(test: &Path)
where
    T: Document,
{
    assert(test, "output.ty.json", doku::to_json::<T>());
}

fn to_json_fmt<T>(test: &Path, fixture: &str, fmt: serde_json::Value)
where
    T: Document,
{
    let fmt =
        serde_json::from_value(fmt).expect("Given formatting is not valid");

    assert(test, fixture, doku::to_json_fmt::<T>(&fmt));
}

fn to_json_val<T>(test: &Path)
where
    T: Document + Serialize + Default,
{
    assert(test, "output.val.json", doku::to_json_val(&T::default()));
}

fn assert(test: &Path, fixture: &str, expected: String) {
    let dir = Path::new("tests").join(test);

    let path = dir.join(fixture);

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

    if actual != expected {
        fs::write(&path_new, &expected).unwrap_or_else(|err| {
            panic!("Couldn't write fixture `{}`: {}", path_new.display(), err)
        });

        let diff = Changeset::new(&actual, &expected, "\n");

        panic!(
            "Found differences between `{}` and `{}`:\n{}",
            path.display(),
            path_new.display(),
            diff,
        );
    }
}
