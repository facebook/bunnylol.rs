/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Black-box tests for the `serve` redirect route. They spawn the real
//! `bunnylol serve` binary and drive it over HTTP, asserting on the redirect
//! `Location`. This is the end-to-end contract of the card forms: the command
//! rides in `cmd`, the user's input rides in `args`, and the server combines
//! them into `"cmd args"` before resolving the target. No internal symbols are
//! referenced, so any valid implementation passes.

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

fn unique_test_dir(test_name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "bunnylol-route-{}-{}-{}",
        test_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    dir
}

fn write_config(xdg_dir: &Path, port: u16) {
    fs::create_dir_all(xdg_dir.join("bunnylol")).expect("create config dir");
    fs::write(
        xdg_dir.join("bunnylol/config.toml"),
        format!(
            r#"default_search = "google"

[history]
enabled = false

[server]
port = {port}
address = "127.0.0.1"
log_level = "critical"
"#
        ),
    )
    .expect("write config");
}

fn free_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0))
        .expect("bind test port")
        .local_addr()
        .expect("read test port")
        .port()
}

struct ServerProcess {
    child: Child,
}

impl Drop for ServerProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_server(xdg_dir: &Path, port: u16) -> ServerProcess {
    let child = Command::new(assert_cmd::cargo::cargo_bin!("bunnylol"))
        .env("XDG_CONFIG_HOME", xdg_dir)
        .arg("serve")
        .arg("--port")
        .arg(port.to_string())
        .arg("--address")
        .arg("127.0.0.1")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn bunnylol server");
    ServerProcess { child }
}

fn http_get(port: u16, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.write_all(
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n").as_bytes(),
    )?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn wait_for_server(server: &mut ServerProcess, port: u16) {
    for _ in 0..50 {
        if let Some(status) = server.child.try_wait().expect("check server status") {
            panic!("server exited before becoming ready: {status}");
        }
        if let Ok(response) = http_get(port, "/health")
            && response.contains("ok")
        {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("server did not become ready");
}

fn redirect_location(response: &str) -> String {
    assert!(
        response.starts_with("HTTP/1.1 303") || response.starts_with("HTTP/1.1 302"),
        "expected redirect response, got:\n{response}"
    );
    response
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("location")
                .then(|| value.trim().to_string())
        })
        .expect("redirect response should include Location header")
}

/// Spawn a server, run `f` with its port, and clean up.
fn with_server<F: FnOnce(u16)>(test_name: &str, f: F) {
    let xdg_dir = unique_test_dir(test_name);
    let port = free_port();
    write_config(&xdg_dir, port);
    let mut server = spawn_server(&xdg_dir, port);
    wait_for_server(&mut server, port);
    f(port);
    drop(server);
    fs::remove_dir_all(&xdg_dir).ok();
}

/// Discover the name of the card's arguments field from the rendered landing
/// page, so the test is agnostic to what the field/parameter is called. We find
/// the `<input>` carrying the mandated "arguments (optional)" placeholder and
/// return its `name=""` value. Panics if no such field exists (e.g. at the base
/// commit, where the cards have no form) — which correctly fails these tests
/// until the feature is implemented.
fn discover_args_param(port: u16) -> String {
    let body = http_get(port, "/").expect("landing page");
    for tag in body.split("<input") {
        if tag.contains(r#"placeholder="arguments (optional)""#)
            && let Some(idx) = tag.find(r#"name=""#)
        {
            let rest = &tag[idx + r#"name=""#.len()..];
            if let Some(end) = rest.find('"') {
                return rest[..end].to_string();
            }
        }
    }
    panic!("no arguments input (placeholder \"arguments (optional)\") found on landing page");
}

// ---------------------------------------------------------------------------
// FAIL_TO_PASS — the arguments field must reach the command (fails at base, where
// the cards have no form / the route ignores the arguments).
// ---------------------------------------------------------------------------

#[test]
fn test_route_passes_args_to_command() {
    with_server("args", |port| {
        let arg = discover_args_param(port);
        let resp = http_get(port, &format!("/?cmd=g&{arg}=hello")).expect("request");
        assert_eq!(
            redirect_location(&resp),
            "https://google.com/search?q=hello"
        );
    });
}

// ---------------------------------------------------------------------------
// PASS_TO_PASS — existing behavior must keep working before AND after the fix.
// ---------------------------------------------------------------------------

#[test]
fn test_route_bare_command_unchanged() {
    with_server("bare", |port| {
        let resp = http_get(port, "/?cmd=g").expect("request");
        assert_eq!(redirect_location(&resp), "https://google.com/search?q=");
    });
}

#[test]
fn test_route_preserves_legacy_single_param() {
    with_server("legacy", |port| {
        let resp = http_get(port, "/?cmd=gh%20facebook/bunnylol.rs").expect("request");
        assert_eq!(
            redirect_location(&resp),
            "https://github.com/facebook/bunnylol.rs"
        );
    });
}

#[test]
fn test_route_serves_landing_page() {
    with_server("landing", |port| {
        let resp = http_get(port, "/").expect("request");
        assert!(
            resp.starts_with("HTTP/1.1 200"),
            "landing page should be 200, got:\n{}",
            resp.lines().next().unwrap_or("")
        );
        assert!(
            resp.contains("Available Commands"),
            "landing page should list commands"
        );
    });
}
