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
// [user_bindings] end-to-end tests
//
// Each test writes an isolated `config.toml` into a per-test temp dir and
// points the CLI at it via `XDG_CONFIG_HOME`. These exercise the full
// load → resolve → emit URL pipeline in a real subprocess.
// =====================================================================

#[test]
#[cfg(feature = "cli")]
fn test_user_binding_url_static() {
    let xdg = write_test_config(
        "url-static",
        r#"
[user_bindings]
cal = { url = "https://calendar.google.com/calendar/u/1/r" }
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
fn test_user_binding_url_templated() {
    let xdg = write_test_config(
        "url-templated",
        r#"
[user_bindings]
jira = { url = "https://corp.atlassian.net/browse/{}" }
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
fn test_user_binding_url_with_description() {
    let xdg = write_test_config(
        "url-described",
        r#"
[user_bindings]
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
fn test_user_binding_command_dispatches_to_builtin() {
    // A Command binding rewrites the input and dispatches into the registry.
    // `work` should fire the GitHub built-in with `mycompany/repo` as args.
    let xdg = write_test_config(
        "cmd-dispatch",
        r#"
[user_bindings]
work = { command = "gh mycompany/repo", description = "Work repo" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("work")
        .assert()
        .success()
        .stdout("https://github.com/mycompany/repo\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_user_binding_command_drops_extra_args() {
    // Command bindings do NOT forward extra args (Q4 in the plan).
    // `work foo bar` resolves to `gh mycompany/repo` — foo bar are dropped.
    let xdg = write_test_config(
        "cmd-drops-args",
        r#"
[user_bindings]
work = { command = "gh mycompany/repo" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("work")
        .arg("foo")
        .arg("bar")
        .assert()
        .success()
        .stdout("https://github.com/mycompany/repo\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_user_binding_no_override_loses_to_builtin() {
    // Without `override = true`, a name collision with a built-in must
    // resolve to the built-in.
    let xdg = write_test_config(
        "no-override",
        r#"
[user_bindings]
gh = { url = "https://example.com/should-be-shadowed" }
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
fn test_user_binding_override_shadows_builtin() {
    // `override = true` opts in to shadowing a built-in command.
    let xdg = write_test_config(
        "override",
        r#"
[user_bindings]
gh = { url = "https://example.com/my-fork", override = true }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stdout("https://example.com/my-fork\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_user_binding_silent_conflict_emits_warning() {
    // Built-in wins on a name collision unless override = true, AND a
    // startup warning is emitted with a hint about the `override` flag.
    let xdg = write_test_config(
        "conflict-warn",
        r#"
[user_bindings]
gh = { url = "https://example.com/should-be-shadowed" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("gh")
        .assert()
        .success()
        .stderr(predicate::str::contains("shadowed").or(predicate::str::contains("conflict")))
        .stderr(predicate::str::contains("override"));
}

#[test]
#[cfg(feature = "cli")]
fn test_user_binding_startup_info_log_no_restart_hint() {
    // Post-PR-#48, the startup info line must NOT mention "restart" — hot
    // reload works now. It should report the load count cleanly.
    let xdg = write_test_config(
        "info-log",
        r#"
[user_bindings]
cal = { url = "https://calendar.google.com/calendar/u/1/r" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("cal")
        .assert()
        .success()
        .stderr(predicate::str::contains("user binding"))
        .stderr(predicate::str::contains("restart").not());
}

#[test]
#[cfg(feature = "cli")]
fn test_no_user_bindings_produces_no_startup_noise() {
    // Empty [user_bindings] and [aliases] → no info line at all.
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
        .stderr(predicate::str::contains("user binding").not())
        .stderr(predicate::str::contains("deprecated").not());
}

#[test]
#[cfg(feature = "cli")]
fn test_unknown_command_still_falls_through_to_search() {
    // Regression guard: an unknown command with no matching binding must
    // continue to fall through to the configured search engine.
    let xdg = write_test_config(
        "search-fallthrough",
        r#"
default_search = "google"

[user_bindings]
cal = { url = "https://calendar.google.com/calendar/u/1/r" }
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
fn test_user_binding_appears_on_list() {
    let xdg = write_test_config(
        "list",
        r#"
[user_bindings]
cal = { url = "https://calendar.google.com/calendar/u/1/r" }
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains("cal"));
}

// =====================================================================
// [aliases] deprecation / migration tests
// =====================================================================

#[test]
#[cfg(feature = "cli")]
fn test_legacy_aliases_still_resolve_via_user_bindings_fold() {
    // [aliases] from before the user_bindings refactor must still work.
    // They're folded into user_bindings as Command variants at load time.
    let xdg = write_test_config(
        "aliases-fold",
        r#"
[aliases]
work = "gh mycompany/repo"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("work")
        .assert()
        .success()
        .stdout("https://github.com/mycompany/repo\n");
}

#[test]
#[cfg(feature = "cli")]
fn test_legacy_aliases_emit_deprecation_warning() {
    let xdg = write_test_config(
        "aliases-deprecation",
        r#"
[aliases]
work = "gh mycompany/repo"
"#,
    );

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("bunnylol");
    cmd.env("XDG_CONFIG_HOME", &xdg)
        .arg("--dry-run")
        .arg("work")
        .assert()
        .success()
        .stderr(predicate::str::contains("[aliases]"))
        .stderr(predicate::str::contains("deprecated"));
}
