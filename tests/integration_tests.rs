use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create test environment with sample files
fn create_test_files() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create nested directory structure
    fs::create_dir_all(base_path.join("src")).unwrap();
    fs::create_dir_all(base_path.join("tests")).unwrap();
    fs::create_dir_all(base_path.join("docs")).unwrap();

    // Create Rust files with various content
    fs::write(
        base_path.join("src/main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}\n\nfn helper() {}\n",
    )
    .unwrap();

    fs::write(
        base_path.join("src/lib.rs"),
        "pub fn calculate(x: i32) -> i32 {\n    x * 2\n}\n\n#[cfg(test)]\nmod tests {}\n",
    )
    .unwrap();

    fs::write(
        base_path.join("tests/integration.rs"),
        "use my_crate::calculate;\n\n#[test]\nfn test_calculate() {\n    assert_eq!(calculate(2), 4);\n}\n",
    )
    .unwrap();

    // Create text files
    fs::write(
        base_path.join("README.md"),
        "# Project\n\nTODO: Add documentation\n\nThis is a test project.\n",
    )
    .unwrap();

    fs::write(
        base_path.join("docs/guide.txt"),
        "Configuration Guide\n\nStep 1: Install\nStep 2: Configure\nTODO: Add more steps\n",
    )
    .unwrap();

    temp_dir
}

#[test]
fn test_missing_arguments() {
    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_invalid_glob_pattern() {
    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg("[invalid")
        .arg("pattern")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid glob pattern"));
}

#[test]
fn test_invalid_regex_pattern() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("[invalid(regex")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid regex pattern"));
}

#[test]
fn test_search_rust_files_for_function() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg(r"fn\s+\w+")
        .assert()
        .success()
        .stdout(predicate::str::contains("main.rs"))
        .stdout(predicate::str::contains("fn main"))
        .stdout(predicate::str::contains("fn helper"))
        .stdout(predicate::str::contains("lib.rs"))
        .stdout(predicate::str::contains("fn calculate"));
}

#[test]
fn test_search_for_todo_comments() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("TODO")
        .assert()
        .success()
        .stdout(predicate::str::contains("README.md"))
        .stdout(predicate::str::contains("TODO: Add documentation"))
        .stdout(predicate::str::contains("guide.txt"))
        .stdout(predicate::str::contains("TODO: Add more steps"));
}

#[test]
fn test_search_specific_directory() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/src/**/*.rs", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg(r"fn\s+\w+")
        .assert()
        .success()
        .stdout(predicate::str::contains("src/main.rs"))
        .stdout(predicate::str::contains("src/lib.rs"));
}

#[test]
fn test_search_with_no_matches() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("THISPATTERNWILLNOTMATCH12345")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_search_case_sensitive() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*", temp_dir.path().display());

    // Search for lowercase 'todo' should not match uppercase 'TODO'
    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(&glob_pattern)
        .arg("^todo$")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    // Search for uppercase 'TODO' should match
    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(&glob_pattern)
        .arg("TODO")
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO"));
}

#[test]
fn test_line_numbers_are_correct() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/src/main.rs", temp_dir.path().display());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("fn main")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // "fn main()" is on line 1
    assert!(stdout.contains("1:"));
    assert!(stdout.contains("fn main"));
}

#[test]
fn test_complex_regex_patterns() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    // Test regex with groups
    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(&glob_pattern)
        .arg(r"(fn|pub fn)\s+\w+")
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main"))
        .stdout(predicate::str::contains("pub fn calculate"));
}

#[test]
fn test_glob_pattern_single_file() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/README.md", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("Project")
        .assert()
        .success()
        .stdout(predicate::str::contains("README.md"))
        .stdout(predicate::str::contains("# Project"));
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let glob_pattern = format!("{}/**/*", temp_dir.path().display());

    Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg("pattern")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_multiline_output_format() {
    let temp_dir = create_test_files();

    // Create a file with multiple matches
    fs::write(
        temp_dir.path().join("multi.rs"),
        "fn first() {}\nfn second() {}\nlet x = 1;\nfn third() {}\n",
    )
    .unwrap();

    let glob_pattern = format!("{}/multi.rs", temp_dir.path().display());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("find-fast"))
        .arg(glob_pattern)
        .arg(r"fn\s+\w+")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show filename once, followed by all matching lines
    assert!(stdout.contains("multi.rs:"));
    assert!(stdout.contains("1:"));
    assert!(stdout.contains("fn first"));
    assert!(stdout.contains("2:"));
    assert!(stdout.contains("fn second"));
    assert!(stdout.contains("4:"));
    assert!(stdout.contains("fn third"));
}
