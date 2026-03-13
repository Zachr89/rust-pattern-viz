use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_analyze_command() {
    let mut cmd = Command::cargo_bin("rpv").unwrap();
    
    cmd.arg("analyze")
        .arg("examples/sample.rs")
        .arg("-f")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("patterns"))
        .stdout(predicate::str::contains("import_suggestions"));
}

#[test]
fn test_html_output() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("report.html");

    let mut cmd = Command::cargo_bin("rpv").unwrap();
    
    cmd.arg("analyze")
        .arg("examples/sample.rs")
        .arg("-f")
        .arg("html")
        .arg("-o")
        .arg(&output_path);
    
    cmd.assert().success();
    
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Rust Pattern Visualization"));
    assert!(content.contains("Detected Patterns"));
}

#[test]
fn test_batch_processing() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("reports");

    let mut cmd = Command::cargo_bin("rpv").unwrap();
    
    cmd.arg("batch")
        .arg("examples")
        .arg("-o")
        .arg(&output_dir)
        .arg("-f")
        .arg("html");
    
    cmd.assert().success();
    
    assert!(output_dir.exists());
    assert!(output_dir.join("sample.html").exists());
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("rpv").unwrap();
    
    cmd.arg("analyze")
        .arg("examples/sample.rs")
        .arg("--verbose");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Analysis Summary"))
        .stdout(predicate::str::contains("confidence"));
}
