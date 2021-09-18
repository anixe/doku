use crate::*;
use std::{fs, path::Path};

pub struct Tests {
    tests: Vec<Test>,
}

impl Tests {
    pub fn discover(dir: impl AsRef<Path>) -> Result<Self> {
        let mut tests = Vec::new();
        let mut pending = vec![dir.as_ref().to_owned()];

        while let Some(dir) = pending.pop() {
            let entries = fs::read_dir(&dir).with_context(|| {
                format!("Couldn't access directory: {}", dir.display())
            })?;

            for entry in entries {
                let entry = entry?;

                let entry_meta = entry.metadata().with_context(|| {
                    format!(
                        "Couldn't access directory entry: {}",
                        entry.path().display(),
                    )
                })?;

                if entry_meta.is_dir() {
                    pending.push(entry.path());
                } else if entry.file_name() == "input.rs" {
                    let test = Test::load(entry.path()).with_context(|| {
                        format!(
                            "Couldn't load test: {}",
                            entry.path().display()
                        )
                    })?;

                    tests.push(test);
                }
            }
        }

        Ok(Self { tests })
    }

    pub fn expand(&self) -> TokenStream {
        TestsMod::build(&self.tests).expand()
    }
}
