//! Compile-time guarantees. Each case under `ui/fail` is a mistake that must not compile.

#[test]
fn ui() {
    // trybuild runs `cargo check` on a set of cases with no passing case
    // and `cargo build` on one with any, so the failing cases go first on
    // their own: a mistake only code generation reports would compile here.
    trybuild::TestCases::new().compile_fail("tests/ui/fail/*.rs");
    trybuild::TestCases::new().pass("tests/ui/pass/*.rs");
}
