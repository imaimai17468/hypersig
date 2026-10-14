//! Compile-time guarantees. Each case under `ui/fail` is a mistake that must not compile.

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/pass/*.rs");
    cases.compile_fail("tests/ui/fail/*.rs");
}
