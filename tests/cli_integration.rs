/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

#[cfg(feature = "cli")]
fn write_test_config(test_name: &str, toml_body: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "bunnylol-it-{}-{}-{}",
        test_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(dir.join("bunnylol")).expect("create temp config dir");
    fs::write(dir.join("bunnylol/config.toml"), toml_body).expect("write config.toml");
    dir
}

#[test]
fn test_cli_help() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("bunnylol"));
}

#[test]
fn test_cli_version() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap());
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_list_commands_flag() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains("gh"))
        .stdout(predicate::str::contains("ig"))
        .stdout(predicate::str::contains("Command"))
        .stdout(predicate::str::contains("Aliases"));
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_list_commands_as_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("gh"))
        .stdout(predicate::str::contains("ig"))
        .stdout(predicate::str::contains("Command"))
        .stdout(predicate::str::contains("Aliases"));
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_dry_run_github() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stdout("https://github.com\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_dry_run_instagram_reels() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--dry-run")
        .arg("ig")
        .arg("reels")
        .assert()
        .success()
        .stdout("https://www.instagram.com/reels/\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_dry_run_github_repo() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.arg("--dry-run")
        .arg("gh")
        .arg("facebook/react")
        .assert()
        .success()
        .stdout("https://github.com/facebook/react\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_missing_config_uses_defaults() {
    let xdg = write_test_config("missing-config", "");
    fs::remove_file(xdg.join("bunnylol/config.toml")).expect("remove config.toml");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stdout("https://github.com\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_cli_invalid_config_exits_with_error() {
    let xdg = write_test_config("invalid-config", "default_search = [\n");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid configuration"));
}
