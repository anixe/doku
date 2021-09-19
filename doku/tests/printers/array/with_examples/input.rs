// run: test_json_ty

#[derive(Doku)]
pub struct Ty {
    f1: Vec<String>,

    #[doku(example = "foo")]
    f2: Vec<String>,

    #[doku(example = "foo, bar")]
    f3: Vec<String>,

    #[doku(example = "[ foo ]")]
    f4: Vec<String>,

    #[doku(example = "[ foo, bar ]")]
    f5: Vec<String>,
}
