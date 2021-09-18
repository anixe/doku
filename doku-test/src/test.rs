use crate::*;
use std::{
    fs,
    path::{Component, PathBuf},
};

pub struct Test {
    dir: PathBuf,
    runs: Vec<syn::ExprCall>,
}

impl Test {
    pub fn load(input: PathBuf) -> Result<Self> {
        let src = fs::read_to_string(&input)
            .context("Couldn't open test's source code")?;

        let mut runs = Vec::new();

        for (line_id, line) in src.lines().enumerate() {
            if !line.starts_with("//") {
                break;
            }

            if let Some(run) = line.strip_prefix("// run: ") {
                let run = syn::parse_str(run).with_context(|| {
                    format!("Couldn't parse annotation at line {}", line_id + 1)
                })?;

                runs.push(run);
            }
        }

        if runs.is_empty() {
            bail!("Test is missing the `run` annotation");
        }

        Ok(Self {
            dir: input.parent().unwrap().to_owned(),
            runs,
        })
    }

    pub fn dirs_and_name(&self) -> (Vec<String>, String) {
        let mut path: Vec<_> = self.path().collect();
        let name = path.pop().unwrap();
        (path, name)
    }

    pub fn path(&self) -> impl Iterator<Item = String> + '_ {
        self.dir.components().skip(2).flat_map(|component| {
            if let Component::Normal(dir) = component {
                Some(dir.to_string_lossy().to_string())
            } else {
                None
            }
        })
    }

    pub fn expand(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let (dir, input) = {
            let path = self.dir.strip_prefix("doku/tests/").unwrap();
            let dir = path.display().to_string();
            let input = path.join("input.rs").display().to_string();
            (dir, input)
        };

        self.runs.iter().enumerate().map(move |(run_idx, run)| {
            let test_fn_name = format_ident!("run_{}", run_idx + 1);
            let test_runner_fn = &run.func;
            let test_runner_args = run.args.iter();

            quote! {
                #[test]
                fn #test_fn_name() {
                    mod source {
                        #[allow(unused_imports)]
                        use super::*;

                        include!(#input);
                    }

                    #test_runner_fn::<source::Ty>(
                        Path::new(#dir),
                        #(#test_runner_args,)*
                    );
                }
            }
        })
    }
}
