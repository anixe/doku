#[test]
fn compiletest() {
    trybuild::TestCases::new().compile_fail("tests/compiletest/**/*.rs");
}
