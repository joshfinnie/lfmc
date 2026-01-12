use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A command-line application"))
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("--username"));
}

#[test]
fn test_empty_api_key() {
    // Test that empty API key is rejected by Config validation
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.env_clear();
    cmd.arg("--api-key")
        .arg("")
        .arg("--username")
        .arg("testuser");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("API key cannot be empty"));
}

#[test]
fn test_empty_username() {
    // Test that empty username is rejected by Config validation
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.env_clear();
    cmd.arg("--api-key")
        .arg("testkey")
        .arg("--username")
        .arg("");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Username cannot be empty"));
}

#[test]
fn test_invalid_period() {
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.arg("--api-key")
        .arg("testkey")
        .arg("--username")
        .arg("testuser")
        .arg("--period")
        .arg("invalid");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));
}

#[test]
fn test_invalid_format() {
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.arg("--api-key")
        .arg("testkey")
        .arg("--username")
        .arg("testuser")
        .arg("--format")
        .arg("invalid");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("lfmc").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_valid_period_values() {
    for period in ["overall", "7day", "1month", "3month", "6month", "12month"] {
        let mut cmd = Command::cargo_bin("lfmc").unwrap();
        cmd.arg("--api-key")
            .arg("testkey")
            .arg("--username")
            .arg("testuser")
            .arg("--period")
            .arg(period);

        // This will fail at the API call, but the period should be accepted
        cmd.assert().failure();
        // The error should NOT be about an invalid period
        let output = String::from_utf8(cmd.output().unwrap().stderr).unwrap();
        assert!(
            !output.contains("invalid value"),
            "Period {} was rejected",
            period
        );
    }
}

#[test]
fn test_valid_format_values() {
    for format in ["social", "json", "csv"] {
        let mut cmd = Command::cargo_bin("lfmc").unwrap();
        cmd.arg("--api-key")
            .arg("testkey")
            .arg("--username")
            .arg("testuser")
            .arg("--format")
            .arg(format);

        // This will fail at the API call, but the format should be accepted
        cmd.assert().failure();
        // The error should NOT be about an invalid format
        let output = String::from_utf8(cmd.output().unwrap().stderr).unwrap();
        assert!(
            !output.contains("invalid value"),
            "Format {} was rejected",
            format
        );
    }
}
