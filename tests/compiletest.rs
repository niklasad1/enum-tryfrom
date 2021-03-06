#![cfg(feature="compiletest")]
extern crate compiletest_rs as compiletest;

// Doesn't seem to work when not in root/tests

#[test]
fn compile_fail() {
    use std::path::PathBuf;
    let mut config = compiletest::default_config();
    config.mode = compiletest::common::Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile-fail");
    config.target_rustcflags = Some("-L target/debug".to_string());

    compiletest::run_tests(&config)
}
