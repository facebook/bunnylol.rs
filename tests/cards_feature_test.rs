/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Integration test for the interactive `serve` cards feature.
//!
//! This is the test the task asks the solution to ship alongside the feature:
//! it exercises the end-to-end behavior of a card's arguments form — typing
//! arguments and submitting opens the combined `cmd args` target through the
//! server's redirect. It spawns the real `bunnylol serve` binary and drives it
//! over HTTP (no browser, no internal symbols).

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

fn free_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0))
        .expect("bind test port")
        .local_addr()
        .expect("read test port")
        .port()
}

fn write_config(xdg_dir: &Path, port: u16) {
    fs::create_dir_all(xdg_dir.join("bunnylol")).expect("create config dir");
    fs::write(
        xdg_dir.join("bunnylol/config.toml"),
        format!(
            "default_search = \"google\"\n\n[history]\nenabled = false\n\n[server]\nport = {port}\naddress = \"127.0.0.1\"\nlog_level = \"critical\"\n"
        ),
    )
    .expect("write config");
}

struct Server {
    child: Child,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
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

fn location(response: &str) -> String {
    response
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("location")
                .then(|| value.trim().to_string())
        })
        .expect("redirect should include a Location header")
}

fn start_server() -> (Server, u16, std::path::PathBuf) {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "bunnylol-cards-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let port = free_port();
    write_config(&dir, port);
    let child = Command::new(assert_cmd::cargo::cargo_bin!("bunnylol"))
        .env("XDG_CONFIG_HOME", &dir)
        .args([
            "serve",
            "--port",
            &port.to_string(),
            "--address",
            "127.0.0.1",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn bunnylol server");
    let mut server = Server { child };
    for _ in 0..50 {
        if let Ok(r) = http_get(port, "/health")
            && r.contains("ok")
        {
            return (server, port, dir);
        }
        if server.child.try_wait().expect("status").is_some() {
            panic!("server exited early");
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("server did not become ready");
}

#[test]
fn test_card_args_open_combined_target() {
    let (server, port, dir) = start_server();
    // Typing "hello" into the `g` card and opening combines into `g hello`.
    let resp = http_get(port, "/?cmd=g&args=hello").expect("request");
    assert_eq!(location(&resp), "https://google.com/search?q=hello");
    drop(server);
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_card_open_bare_command() {
    let (server, port, dir) = start_server();
    // Clicking the title (no args) opens the bare command.
    let resp = http_get(port, "/?cmd=g").expect("request");
    assert_eq!(location(&resp), "https://google.com/search?q=");
    drop(server);
    fs::remove_dir_all(&dir).ok();
}
