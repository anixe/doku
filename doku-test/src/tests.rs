use crate::Test;
use std::path::Path;

pub fn discover(dir: impl AsRef<Path>) -> Vec<Test> {
    let mut tests = Vec::new();
    let mut pending = vec![dir.as_ref().to_owned()];

    while let Some(dir) = pending.pop() {
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("Couldn't access directory `{}`: {}", dir.display(), err));

        for entry in entries {
            let entry = entry.unwrap();

            let entry_meta = entry
                .metadata()
                .unwrap_or_else(|err| panic!("Couldn't access entry `{}`: {}", entry.path().display(), err));

            if entry_meta.is_dir() {
                pending.push(entry.path());
            } else if entry.file_name() == "input.rs" {
                tests.push(Test::load(entry.path()));
            }
        }
    }

    tests.sort_by_key(|test| test.dir().to_owned());
    tests
}
