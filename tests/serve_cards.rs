/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Black-box tests for the interactive `serve` landing-page cards. They render
//! the page to an HTML string via the public `render_landing_page_html` and
//! assert on observable markup — no browser, no running server. URLs are checked
//! by their `?cmd=<command>` target, accepting either relative or absolute hrefs
//! (an implementation choice the feature does not constrain).

#![cfg(feature = "server")]

use bunnylol::config::UserBinding;
use bunnylol::server::web::render_landing_page_html;
use bunnylol::{BunnylolCommandRegistry, BunnylolConfig};

// ---------------------------------------------------------------------------
// FAIL_TO_PASS — new interactive behavior (must fail at base, pass after fix)
// ---------------------------------------------------------------------------

/// Each command card's title must be a link that opens the bare command via the
/// server's `/?cmd=` redirect.
#[test]
fn test_builtin_card_links_to_command() {
    let html = render_landing_page_html(&BunnylolConfig::default());
    assert!(
        html.contains(r#"?cmd=threads""#),
        "threads card title should link to ?cmd=threads"
    );
}

/// Every command card must be interactive — not just one. There must be at least
/// as many `?cmd=` link targets as there are registered commands.
#[test]
fn test_all_builtin_commands_are_linked() {
    let html = render_landing_page_html(&BunnylolConfig::default());
    let command_count = BunnylolCommandRegistry::get_all_commands().len();
    let link_count = html.matches("?cmd=").count();
    assert!(
        link_count >= command_count,
        "expected >= {command_count} command links, found {link_count}"
    );
}

/// Each card carries a JS-free GET form: the command rides in a hidden `cmd`
/// field (not editable), the visible field is args-only with an
/// "arguments (optional)" placeholder, a submit button opens it (so Enter works
/// too), and a reset control clears the args.
#[test]
fn test_builtin_card_has_arg_form() {
    let html = render_landing_page_html(&BunnylolConfig::default());
    // A real GET form (the command must travel to the server, not be combined in
    // client-side JavaScript). A GET form may omit the default `method`, so we do
    // not pin `method="get"` literally — only that it is not a POST form.
    assert!(html.contains("<form"), "card must contain a <form>");
    assert!(
        !html.contains(r#"method="post""#),
        "card form must submit via GET, not POST"
    );
    // Command in a hidden field carrying its value.
    assert!(
        html.contains(r#"type="hidden""#),
        "command should be carried in a hidden field"
    );
    assert!(
        html.contains(r#"name="cmd""#),
        "hidden command field must be named cmd"
    );
    assert!(
        html.contains(r#"value="threads""#),
        "hidden field must carry the command value (threads)"
    );
    // Args-only visible field, identified by its mandated placeholder (its
    // name/param is an implementation choice, so we don't pin it here — the
    // route test discovers it from the form).
    assert!(
        html.contains(r#"placeholder="arguments (optional)""#),
        "the args input must show an 'arguments (optional)' placeholder"
    );
    // Submit (Enter opens) and reset (clear) controls.
    assert!(
        html.contains(r#"type="submit""#),
        "an open/submit button must exist so Enter submits the form"
    );
    assert!(
        html.contains(r#"type="reset""#),
        "a clear (reset) control must be present to empty the args field"
    );
    // The command must NOT be pre-filled into an editable field.
    assert!(
        !html.contains(r#"value="threads ""#),
        "command must not be seeded into an editable input"
    );
}

/// The solution must ship its own test for the feature (a repo convention). We
/// look for an agent-authored test file under `tests/` — excluding the base
/// test files and these gold files — that contains a real `#[test]` with an
/// assertion and exercises the cards/args feature surface. This is a static
/// check: it never runs the agent's test.
#[test]
fn test_agent_added_feature_test() {
    use std::fs;
    // Test files that exist at the base commit or are installed by the gold
    // test_patch — these do NOT count as the solution's own feature test.
    let excluded = [
        "serve_cards.rs",
        "serve_route.rs",
        "server_e2e.rs",
        "cli_integration.rs",
    ];
    let mut found = false;
    for entry in fs::read_dir("tests").expect("a tests/ directory") {
        let path = entry.expect("dir entry").path();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if !name.ends_with(".rs") || excluded.contains(&name.as_str()) {
            continue;
        }
        let src = fs::read_to_string(&path).unwrap_or_default();
        let has_test = src.contains("#[test]");
        let has_assert = src.contains("assert");
        let exercises_feature = src.contains("render_landing_page_html")
            || src.contains("?cmd=")
            || src.contains("/?cmd")
            || src.contains("serve");
        if has_test && has_assert && exercises_feature {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "the solution must add an integration test (e.g. tests/cards_feature_test.rs) with a \
         #[test] that exercises the cards/args feature — none was found"
    );
}

// ---------------------------------------------------------------------------
// PASS_TO_PASS — regressions that must hold before AND after the fix
// ---------------------------------------------------------------------------

/// The page still renders for a default config and shows the commands section.
#[test]
fn test_landing_page_renders_with_default_config() {
    let html = render_landing_page_html(&BunnylolConfig::default());
    assert!(html.contains("bunnylol"), "page title should be present");
    assert!(
        html.contains("Available Commands"),
        "commands section heading should be present"
    );
}

/// The page still lists built-in command names.
#[test]
fn test_landing_page_lists_command_names() {
    let html = render_landing_page_html(&BunnylolConfig::default());
    assert!(html.contains("threads"), "threads card should be listed");
    assert!(html.contains("Available Commands"));
}

/// The existing "set as default search engine" example links are preserved.
#[test]
fn test_existing_example_links_present() {
    let mut config = BunnylolConfig::default();
    config.server.server_display_url = Some("https://bunny.example.com".to_string());
    let html = render_landing_page_html(&config);
    assert!(
        html.contains("?cmd=%s"),
        "the %s search-engine example URL should still be shown"
    );
    assert!(
        html.contains("cmd=gh facebook/bunnylol.rs"),
        "the gh example URL should still be shown"
    );
}

/// User-binding cards still render (guards the user_bindings render path).
#[test]
fn test_user_binding_card_still_renders() {
    let mut config = BunnylolConfig::default();
    config.user_bindings.insert(
        "myalias-p2p".to_string(),
        UserBinding::Url {
            url: "https://example.com/myalias-p2p".to_string(),
            description: None,
            override_builtin: false,
        },
    );
    let html = render_landing_page_html(&config);
    assert!(
        html.contains("myalias-p2p"),
        "user binding card should be rendered"
    );
    assert!(
        html.contains("User Bindings"),
        "user bindings section heading"
    );
}
