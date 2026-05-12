/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Create a fresh, isolated XDG config directory for a single test.
/// Returns the parent dir (to be passed as `XDG_CONFIG_HOME`) and writes
/// the provided TOML to `<parent>/bunnylol/config.toml`.
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

// =====================================================================
// Custom [bindings] end-to-end tests
//
// Each test writes an isolated `config.toml` into a per-test temp dir and
// points the CLI at it via `XDG_CONFIG_HOME`. These exercise the full
// load → resolve → emit URL pipeline in a real subprocess.
// =====================================================================

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_static_url() {
    let xdg = write_test_config(
        "static",
        r#"
[bindings]
cal = "https://calendar.google.com/calendar/u/1/r"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("cal")
        .assert()
        .success()
        .stdout("https://calendar.google.com/calendar/u/1/r\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_templated() {
    let xdg = write_test_config(
        "templated",
        r#"
[bindings]
jira = "https://corp.atlassian.net/browse/{}"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("jira")
        .arg("PROJ-123")
        .assert()
        .success()
        .stdout("https://corp.atlassian.net/browse/PROJ-123\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_detailed_form() {
    let xdg = write_test_config(
        "detailed",
        r#"
[bindings]
notion = { url = "https://www.notion.so/{}", description = "Notion page" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("notion")
        .arg("abc123")
        .assert()
        .success()
        .stdout("https://www.notion.so/abc123\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_conflict_with_builtin_does_not_override() {
    // A user binding for `gh` must NOT override the built-in GitHub command.
    let xdg = write_test_config(
        "conflict",
        r#"
[bindings]
gh = "https://example.com/should-be-shadowed"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .arg("facebook/react")
        .assert()
        .success()
        .stdout("https://github.com/facebook/react\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_conflict_emits_startup_warning() {
    // Surface #2: when a custom binding conflicts with a built-in, the
    // process must emit a stderr warning at startup.
    let xdg = write_test_config(
        "conflict-warn",
        r#"
[bindings]
gh = "https://example.com/should-be-shadowed"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stderr(predicate::str::contains("shadowed").or(predicate::str::contains("conflict")));
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_startup_info_log_lists_count() {
    // Surface #2: a non-empty [bindings] table must produce an info line
    // mentioning that custom bindings were loaded AND a restart hint.
    let xdg = write_test_config(
        "info-log",
        r#"
[bindings]
cal = "https://calendar.google.com/calendar/u/1/r"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("cal")
        .assert()
        .success()
        .stderr(predicate::str::contains("custom binding"))
        .stderr(predicate::str::contains("restart"));
}

#[test]
#[cfg(feature = "cli")]
fn test_no_custom_bindings_produces_no_startup_noise() {
    // Regression guard: empty [bindings] (or none at all) must not print
    // the custom-bindings info line — only print when N > 0.
    let xdg = write_test_config(
        "no-bindings",
        r#"
default_search = "google"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stderr(predicate::str::contains("custom binding").not());
}

#[test]
#[cfg(feature = "cli")]
fn test_unknown_command_still_falls_through_to_search() {
    // Regression guard: an unknown command with no matching custom binding
    // must continue to fall through to the configured search engine.
    let xdg = write_test_config(
        "search-fallthrough",
        r#"
default_search = "google"

[bindings]
cal = "https://calendar.google.com/calendar/u/1/r"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("definitelyunknownxyz")
        .assert()
        .success()
        .stdout(predicate::str::contains("google.com/search"));
}

#[test]
#[cfg(feature = "cli")]
fn test_custom_binding_appears_on_list_or_bindings_command() {
    // The user-defined binding should be visible somewhere in the
    // commands listing (either as a row or as a User Bindings section).
    let xdg = write_test_config(
        "list",
        r#"
[bindings]
cal = "https://calendar.google.com/calendar/u/1/r"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains("cal"));
}
