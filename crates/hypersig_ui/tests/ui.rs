//! Compile-time guarantees. Each case under `ui/fail` is a mistake that must not compile.

#[test]
fn ui() {
    trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
}
