use std::path::{Path, PathBuf};

pub struct Test {
    dir: PathBuf,
    run: Vec<String>,
}

impl Test {
    pub fn load(input: PathBuf) -> Self {
        let src = std::fs::read_to_string(&input)
            .unwrap_or_else(|err| format!("Couldn't open test's input file `{}`: {}", input.display(), err));

        let mut run = None;

        for line in src.lines() {
            if !line.starts_with("//") {
                break;
            }

            if let Some(line) = line.strip_prefix("// run: ") {
                run = Some(line.split(',').map(str::trim).map(ToOwned::to_owned).collect());
            }
        }

        Self {
            dir: input.parent().unwrap().to_owned(),
            run: run.unwrap_or_else(|| panic!("Test `{}` doesn't have the `run` property", input.display())),
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn run(&self) -> &[String] {
        &self.run
    }
}
