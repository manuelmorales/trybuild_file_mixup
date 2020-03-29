use trybuild::TestCases;

#[test]
fn test_1() {
    TestCases::new().compile_fail("compile_failures/1.rs");
}

#[test]
fn test_2() {
    TestCases::new().compile_fail("compile_failures/2.rs");
}
