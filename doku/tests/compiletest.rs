#[test]
#[allow(unused_attributes)]
fn compiletest() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compiletest/**/*.rs");
}
