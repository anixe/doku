// run: test_json_ty

#[derive(Doku)]
pub struct Ty {
    // This is `f1`
    f1: String,

    /// This is `f2`
    f2: String,

    /// This is `f3`
    /// ... and it's
    /// ... multiline!
    f3: String,
}
