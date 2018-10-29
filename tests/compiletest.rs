extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

// NOTE: you may need to always `rm -rf target` before running `cargo test`
// laumann/compiletest-rs#114
fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    // Try populating rustflags directly to avoid compiletest-rs #81
    // config.link_deps(); // Populate config.target_rustcflags with dependencies on the path
    config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
    config.clean_rmeta(); // If your tests import the parent crate, this helps with E0464

    compiletest::run_tests(&config);
}

#[test]
fn compile_test_run_pass() {
    run_mode("run-pass");
}

#[test]
// Ignoring until we get reliable and stable span info from the compiler in all cases
#[ignore]
fn compile_test_compile_fail() {
    run_mode("compile-fail");
}
