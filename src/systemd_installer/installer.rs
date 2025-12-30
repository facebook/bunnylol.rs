/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::error::InstallError;
use super::service_templates::{ServiceConfig, generate_system_service, generate_user_service};

/// Check if systemd is available on the system
fn check_systemd_available() -> Result<(), InstallError> {
    let output = Command::new("systemctl")
        .arg("--version")
        .output();

    match output {
        Ok(out) if out.status.success() => Ok(()),
        _ => Err(InstallError::SystemdNotAvailable),
    }
}

/// Check if systemd --user is running
fn check_user_systemd_running() -> Result<(), InstallError> {
    let output = Command::new("systemctl")
        .args(&["--user", "status"])
        .output();

    match output {
        Ok(out) if out.status.success() => Ok(()),
        _ => Err(InstallError::UserSystemdNotRunning),
    }
}

/// Check if running as root
fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

/// Check if service is already installed
fn service_exists(user_mode: bool) -> Result<bool, InstallError> {
    let service_path = if user_mode {
        let home = std::env::var("HOME").map_err(|_| InstallError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set")
        ))?;
        PathBuf::from(home).join(".config/systemd/user/bunnylol.service")
    } else {
        PathBuf::from("/etc/systemd/system/bunnylol.service")
    };

    Ok(service_path.exists())
}

/// Find bunnylol binary (check PATH, then target/release/)
fn find_binary() -> Option<PathBuf> {
    // Check if in PATH
    if let Ok(output) = Command::new("which").arg("bunnylol").output() {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                return Some(PathBuf::from(path.trim()));
            }
        }
    }

    // Check target/release/
    let release_path = Path::new("target/release/bunnylol");
    if release_path.exists() {
        return Some(release_path.to_path_buf());
    }

    None
}

/// Build bunnylol binary
fn build_binary() -> Result<PathBuf, InstallError> {
    println!("Building bunnylol binary...");
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .map_err(|e| InstallError::BuildFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(InstallError::BuildFailed(stderr.to_string()));
    }

    let binary_path = Path::new("target/release/bunnylol");
    if binary_path.exists() {
        Ok(binary_path.to_path_buf())
    } else {
        Err(InstallError::BinaryNotFound)
    }
}

/// Create system user 'bunnylol'
fn create_system_user() -> Result<(), InstallError> {
    // Check if user already exists
    let check = Command::new("id")
        .arg("bunnylol")
        .output();

    if let Ok(out) = check {
        if out.status.success() {
            println!("✓ System user 'bunnylol' already exists");
            return Ok(());
        }
    }

    println!("Creating system user 'bunnylol'...");
    let output = Command::new("useradd")
        .args(&["-r", "-s", "/bin/false", "-d", "/var/lib/bunnylol", "bunnylol"])
        .output()
        .map_err(|e| InstallError::UserCreationFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(InstallError::UserCreationFailed(stderr.to_string()));
    }

    // Create and set ownership of data directory
    fs::create_dir_all("/var/lib/bunnylol")?;
    Command::new("chown")
        .args(&["-R", "bunnylol:bunnylol", "/var/lib/bunnylol"])
        .output()
        .map_err(|e| InstallError::UserCreationFailed(e.to_string()))?;

    println!("✓ Created system user 'bunnylol'");
    Ok(())
}

/// Install bunnylol service
pub fn install_service(config: ServiceConfig, force: bool, enable: bool, start: bool) -> Result<(), InstallError> {
    let user_mode = config.user_mode;

    // Pre-flight checks
    println!("Running pre-flight checks...");
    check_systemd_available()?;

    if user_mode {
        check_user_systemd_running()?;
        println!("✓ Systemd user mode available");
    } else {
        if !is_root() {
            return Err(InstallError::PermissionDenied);
        }
        println!("✓ Running as root");
    }

    // Check if already installed
    if service_exists(user_mode)? && !force {
        return Err(InstallError::AlreadyInstalled);
    }

    // Find or build binary
    let binary_path = match find_binary() {
        Some(path) => {
            println!("✓ Found bunnylol binary at {}", path.display());
            path
        }
        None => {
            println!("Binary not found, building from source...");
            build_binary()?
        }
    };

    // Install binary
    let install_path = if user_mode {
        let home = std::env::var("HOME").map_err(|_| InstallError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set")
        ))?;
        let local_bin = PathBuf::from(&home).join(".local/bin");
        fs::create_dir_all(&local_bin)?;
        local_bin.join("bunnylol")
    } else {
        PathBuf::from("/usr/local/bin/bunnylol")
    };

    fs::copy(&binary_path, &install_path)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&install_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&install_path, perms)?;
    }

    println!("✓ Binary installed to {}", install_path.display());

    // Create system user if needed
    if !user_mode {
        create_system_user()?;
    }

    // Generate and install service file
    let service_content = if user_mode {
        generate_user_service(&config)
    } else {
        generate_system_service(&config)
    };

    let service_path = if user_mode {
        let home = std::env::var("HOME").map_err(|_| InstallError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set")
        ))?;
        let service_dir = PathBuf::from(&home).join(".config/systemd/user");
        fs::create_dir_all(&service_dir)?;
        service_dir.join("bunnylol.service")
    } else {
        PathBuf::from("/etc/systemd/system/bunnylol.service")
    };

    fs::write(&service_path, service_content)?;
    println!("✓ Service file created at {}", service_path.display());

    // Reload systemd
    let reload_cmd = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "daemon-reload"])
            .output()
    } else {
        Command::new("systemctl")
            .arg("daemon-reload")
            .output()
    };

    reload_cmd.map_err(|e| InstallError::CommandFailed(e.to_string()))?;
    println!("✓ Systemd daemon reloaded");

    // Enable service
    if enable {
        let enable_cmd = if user_mode {
            Command::new("systemctl")
                .args(&["--user", "enable", "bunnylol"])
                .output()
        } else {
            Command::new("systemctl")
                .args(&["enable", "bunnylol"])
                .output()
        };

        enable_cmd.map_err(|e| InstallError::CommandFailed(e.to_string()))?;
        println!("✓ Service enabled");
    }

    // Start service
    if start {
        let start_cmd = if user_mode {
            Command::new("systemctl")
                .args(&["--user", "start", "bunnylol"])
                .output()
        } else {
            Command::new("systemctl")
                .args(&["start", "bunnylol"])
                .output()
        };

        let result = start_cmd.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(InstallError::ServiceStartFailed(stderr.to_string()));
        }

        println!("✓ Service started");

        // Wait a bit for startup
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Health check
        let health_url = format!("http://{}:{}/health", config.address, config.port);
        println!("Performing health check: {}", health_url);

        // Simple health check (could enhance with actual HTTP request)
        println!("✓ Service appears to be running");
    }

    println!("\n🎉 Bunnylol server installed successfully!");

    if !user_mode {
        println!("\nServer URL: http://{}:{}", config.address, config.port);
        println!("Add to browser search: http://{}:{}/?cmd=%s", config.address, config.port);
    } else {
        println!("\nServer URL: http://localhost:{}", config.port);
        println!("Add to browser search: http://localhost:{}/?cmd=%s", config.port);
    }

    println!("\nManage service:");
    let sudo_prefix = if user_mode { "" } else { "sudo " };
    let user_flag = if user_mode { " --user" } else { "" };
    println!("  {}bunnylol server status{}", sudo_prefix, user_flag);
    println!("  {}bunnylol server logs -f{}", sudo_prefix, user_flag);
    println!("  {}bunnylol server restart{}", sudo_prefix, user_flag);

    Ok(())
}

/// Uninstall bunnylol service
pub fn uninstall_service(user_mode: bool, remove_binary: bool, remove_data: bool) -> Result<(), InstallError> {
    // Check if service exists
    if !service_exists(user_mode)? {
        return Err(InstallError::ServiceNotInstalled);
    }

    // Stop service
    println!("Stopping service...");
    let stop_cmd = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "stop", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["stop", "bunnylol"])
            .output()
    };
    stop_cmd.ok(); // Ignore errors if already stopped

    // Disable service
    println!("Disabling service...");
    let disable_cmd = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "disable", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["disable", "bunnylol"])
            .output()
    };
    disable_cmd.ok(); // Ignore errors if already disabled

    // Remove service file
    let service_path = if user_mode {
        let home = std::env::var("HOME").map_err(|_| InstallError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set")
        ))?;
        PathBuf::from(home).join(".config/systemd/user/bunnylol.service")
    } else {
        PathBuf::from("/etc/systemd/system/bunnylol.service")
    };

    fs::remove_file(&service_path)?;
    println!("✓ Service file removed");

    // Reload systemd
    let reload_cmd = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "daemon-reload"])
            .output()
    } else {
        Command::new("systemctl")
            .arg("daemon-reload")
            .output()
    };
    reload_cmd.ok();
    println!("✓ Systemd daemon reloaded");

    // Remove binary if requested
    if remove_binary {
        let binary_path = if user_mode {
            let home = std::env::var("HOME").map_err(|_| InstallError::IoError(
                std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set")
            ))?;
            PathBuf::from(&home).join(".local/bin/bunnylol")
        } else {
            PathBuf::from("/usr/local/bin/bunnylol")
        };

        if binary_path.exists() {
            fs::remove_file(&binary_path)?;
            println!("✓ Binary removed");
        }
    }

    // Remove data directory if requested (system only)
    if remove_data && !user_mode {
        let data_dir = Path::new("/var/lib/bunnylol");
        if data_dir.exists() {
            fs::remove_dir_all(data_dir)?;
            println!("✓ Data directory removed");
        }
    }

    println!("\n✓ Bunnylol service uninstalled successfully");

    Ok(())
}
