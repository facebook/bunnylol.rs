use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

fn unique_test_dir(test_name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "bunnylol-e2e-{}-{}-{}",
        test_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    dir
}

fn config_path(xdg_dir: &Path) -> PathBuf {
    xdg_dir.join("bunnylol/config.toml")
}

fn write_config(xdg_dir: &Path, default_search: &str, port: u16) {
    fs::create_dir_all(xdg_dir.join("bunnylol")).expect("create config dir");
    fs::write(
        config_path(xdg_dir),
        format!(
            r#"default_search = "{default_search}"

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

fn write_invalid_config(xdg_dir: &Path) {
    fs::write(config_path(xdg_dir), "default_search = [\n").expect("write invalid config");
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
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("bunnylol"));
    let child = command
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

fn assert_redirect_starts_with(port: u16, expected_prefix: &str) {
    let response = http_get(port, "/?cmd=e2e-reload-query").expect("request redirect");
    let location = redirect_location(&response);
    assert!(
        location.starts_with(expected_prefix),
        "expected redirect to start with {expected_prefix}, got {location}"
    );
}

#[test]
#[cfg(feature = "server")]
fn test_server_reloads_config_mtime_and_keeps_last_valid_config() {
    let xdg_dir = unique_test_dir("config-reload");
    let port = free_port();
    write_config(&xdg_dir, "google", port);

    let mut server = spawn_server(&xdg_dir, port);
    wait_for_server(&mut server, port);

    assert_redirect_starts_with(port, "https://www.google.com/search?q=");

    std::thread::sleep(Duration::from_millis(1100));
    write_config(&xdg_dir, "ddg", port);
    assert_redirect_starts_with(port, "https://duckduckgo.com/?q=");

    std::thread::sleep(Duration::from_millis(1100));
    write_invalid_config(&xdg_dir);
    assert_redirect_starts_with(port, "https://duckduckgo.com/?q=");

    fs::remove_dir_all(&xdg_dir).ok();
}
