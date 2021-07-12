use std::{
    fs,
    process::{Command, Stdio},
};

use difference::Changeset;

struct Readme {
    content: String,
}

impl Readme {
    fn new() -> Self {
        let content = fs::read_to_string("../README.md").expect("Couldn't open README.md");

        Self { content }
    }

    fn example_code(&self, id: &str) -> String {
        self.read_fenced_block(&format!("# doku/examples/{}.rs", id))
            .unwrap_or_else(|| panic!("Couldn't find code for example `{}`", id))
    }

    fn example_output(&self, id: &str) -> String {
        self.read_fenced_block(&format!("// cd doku && cargo run --example {}", id))
            .unwrap_or_else(|| panic!("Couldn't find output for example `{}`", id))
    }

    fn read_fenced_block(&self, tag: &str) -> Option<String> {
        let mut recording = false;
        let mut result = String::default();

        for line in self.content.lines() {
            if recording {
                // Ignore empty lines at the beginning of the example
                if result.is_empty() && line.is_empty() {
                    continue;
                }

                if line == "```" {
                    return Some(result);
                }

                result.push_str(line);
                result.push_str("\n");
            } else {
                if line == tag {
                    recording = true;
                }
            }
        }

        None
    }
}

struct Examples;

impl Examples {
    fn actual_code(id: &str) -> String {
        fs::read_to_string(format!("examples/{}.rs", id))
            .unwrap_or_else(|err| format!("Couldn't open example `{}`: {}", id, err))
    }

    fn actual_output(id: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(id)
            .stdout(Stdio::piped())
            .output()
            .unwrap_or_else(|err| panic!("Couldn't compile example `{}`: {}", id, err));

        if !output.status.success() {
            panic!(
                "Couldn't compile example `{}`: {}",
                id,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}

#[test]
fn examples() {
    let readme = Readme::new();

    for &id in &["doku", "serde"] {
        let readme_code = readme.example_code(id);
        let actual_code = Examples::actual_code(id);

        if readme_code != actual_code {
            let diff = Changeset::new(&readme_code, &actual_code, "\n");
            panic!("Found code mismatch for example `{}`:\n{}", id, diff);
        }

        let readme_output = readme.example_output(id);
        let actual_output = Examples::actual_output(id);

        if readme_output != actual_output {
            let diff = Changeset::new(&readme_output, &actual_output, "\n");
            panic!("Found output mismatch for example `{}`:\n{}", id, diff);
        }
    }
}
