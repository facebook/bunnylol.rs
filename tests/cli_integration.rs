use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("bunnylol"));
}

#[test]
fn test_cli_version() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_cli_list_commands_flag() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains("gh"))
        .stdout(predicate::str::contains("ig"))
        .stdout(predicate::str::contains("Command"))
        .stdout(predicate::str::contains("Aliases"));
}

#[test]
fn test_cli_list_commands_as_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("gh"))
        .stdout(predicate::str::contains("ig"))
        .stdout(predicate::str::contains("Command"))
        .stdout(predicate::str::contains("Aliases"));
}

#[test]
fn test_cli_dry_run_github() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stdout("https://github.com\n");
}

#[test]
fn test_cli_dry_run_instagram_reels() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--dry-run")
        .arg("ig")
        .arg("reels")
        .assert()
        .success()
        .stdout("https://www.instagram.com/reels/\n");
}

#[test]
fn test_cli_dry_run_github_repo() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol-cli");
    cmd.arg("--dry-run")
        .arg("gh")
        .arg("facebook/react")
        .assert()
        .success()
        .stdout("https://github.com/facebook/react\n");
}
