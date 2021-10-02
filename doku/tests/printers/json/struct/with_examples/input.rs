// run: to_json()

#[derive(Document)]
pub struct Ty {
    /// Comment for f1
    #[doku(example = "f1-value")]
    f1: String,

    /// Comment for f2
    #[doku(example = "f2-value")]
    f2: Option<String>,

    /// Comment for f3
    #[doku(example = "f3-value")]
    f3: Option<NestedStringA>,

    /// Comment for f4
    #[doku(example = "f4-value")]
    f4: Option<NestedStringB>,

    /// Comment for f5
    #[doku(example = "f5-value")]
    f5: Option<NestedStringC>,

    /// Comment for f6
    #[doku(example = "f6-value")]
    f6: Option<Option<Option<String>>>,
}

#[derive(Document)]
struct NestedStringA(String);

#[derive(Document)]
struct NestedStringB(NestedStringA);

#[derive(Document)]
#[doku(transparent)]
struct NestedStringC {
    value: NestedStringB,
}
