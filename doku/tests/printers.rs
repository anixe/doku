#![allow(dead_code)]

use difference::Changeset;
use doku::prelude::*;
use serde::*;
use std::fs;
use std::path::Path;

doku_test::printers!();

fn test_json_ty<T: doku::ty::Provider>(dir: &Path) {
    assert(dir, "output.ty.json", doku::to_json::<T>());
}

fn test_json_val<T: doku::ty::Provider + doku::val::Provider + Default>(dir: &Path) {
    assert(dir, "output.val.json", doku::to_json_val(&T::default()));
}

fn assert(test: &Path, fixture: &str, expected: String) {
    let dir = Path::new("tests").join(test);
    let path = dir.join(fixture);
    let path_new = path.with_extension(format!("{}.new", path.extension().unwrap().to_string_lossy()));

    if path_new.exists() {
        fs::remove_file(&path_new)
            .unwrap_or_else(|err| panic!("Couldn't remove new-fixture `{}`: {}", path_new.display(), err))
    }

    let actual = if path.exists() {
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("Couldn't read fixture `{}`: {}", path.display(), err))
    } else {
        Default::default()
    };

    if actual != expected {
        fs::write(&path_new, &expected)
            .unwrap_or_else(|err| panic!("Couldn't write fixture `{}`: {}", path_new.display(), err));

        let diff = Changeset::new(&actual, &expected, "\n");

        panic!(
            "Found differences between `{}` and `{}`:\n{}",
            path.display(),
            path_new.display(),
            diff,
        );
    }
}
