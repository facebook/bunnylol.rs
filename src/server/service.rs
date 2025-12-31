/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt;

#[cfg(target_os = "linux")]
use service_manager::*;
#[cfg(target_os = "linux")]
use std::ffi::OsString;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::process::Command;

/// Service label used for systemd
pub const SERVICE_LABEL: &str = "bunnylol";

/// Service name used in systemctl/journalctl commands
pub const SERVICE_NAME: &str = "bunnylol";

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug)]
pub enum ServiceError {
    ServiceManagerError(String),
    BinaryNotFound,
    ServiceStartFailed(String),
    ConfigError(String),
    IoError(std::io::Error),
    UnsupportedPlatform,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ServiceManagerError(msg) => {
                write!(f, "service manager error: {}", msg)
            }
            ServiceError::BinaryNotFound => {
                write!(
                    f,
                    "bunnylol binary not found in PATH\n\n\
                    Please install bunnylol first:\n  \
                    cargo install bunnylol\n\n\
                    Or install from the current directory:\n  \
                    cargo install --path ."
                )
            }
            ServiceError::ServiceStartFailed(msg) => {
                write!(f, "service installed but failed to start: {}", msg)
            }
            ServiceError::ConfigError(msg) => {
                write!(f, "config error: {}", msg)
            }
            ServiceError::IoError(e) => {
                write!(f, "I/O error: {}", e)
            }
            ServiceError::UnsupportedPlatform => {
                write!(
                    f,
                    "Native service installation is only supported on Linux (systemd).\n\n\
                    For macOS and Windows, please use Docker instead:\n  \
                    docker compose up -d\n\n\
                    Or run the server directly:\n  \
                    bunnylol serve"
                )
            }
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::IoError(err)
    }
}

// ============================================================================
// Service Configuration
// ============================================================================

/// Configuration for service installation
pub struct ServiceConfig {
    pub port: u16,
    pub address: String,
    pub log_level: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            address: "127.0.0.1".to_string(), // Localhost only by default (secure)
            log_level: "normal".to_string(),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Common helper to set up service manager with label (Linux systemd only)
#[cfg(target_os = "linux")]
fn setup_manager() -> Result<(Box<dyn ServiceManager>, ServiceLabel), ServiceError> {
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    manager.set_level(ServiceLevel::System).map_err(|e| {
        ServiceError::ServiceManagerError(format!("Failed to set service level: {}", e))
    })?;

    let label: ServiceLabel = SERVICE_LABEL
        .parse()
        .map_err(|e| ServiceError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    Ok((manager, label))
}

// ============================================================================
// Service Lifecycle Functions
// ============================================================================

/// Install bunnylol service using systemd (Linux only)
#[cfg(target_os = "linux")]
pub fn install_systemd_service(config: ServiceConfig) -> Result<(), ServiceError> {
    println!("Installing bunnylol system service...");
    println!("Platform: Linux (systemd)");
    println!();

    let binary_path = which::which("bunnylol").map_err(|_| ServiceError::BinaryNotFound)?;
    println!("✓ Found bunnylol binary: {}", binary_path.display());
    println!(
        "✓ Service file will be created at: /etc/systemd/system/{}.service",
        SERVICE_NAME
    );

    // Create or update config file at /etc/bunnylol/config.toml
    let system_config_path = PathBuf::from("/etc/bunnylol/config.toml");

    // Create directory if needed
    std::fs::create_dir_all("/etc/bunnylol")
        .map_err(|e| ServiceError::ConfigError(format!("Failed to create /etc/bunnylol: {}", e)))?;

    use crate::config::BunnylolConfig;

    if system_config_path.exists() {
        println!("✓ Found existing config file: /etc/bunnylol/config.toml");

        // Load existing config
        let mut existing_config = BunnylolConfig::load().map_err(|e| {
            ServiceError::ConfigError(format!("Failed to load existing config: {}", e))
        })?;

        let current_address = existing_config.server.address.clone();
        println!("  Current address: {}", current_address);
        println!("  New address:     {}", config.address);

        if current_address == config.address {
            println!("✓ Config already has correct address, no changes needed");
        } else {
            // Update only the address field, preserve everything else
            existing_config.server.address = config.address.clone();

            println!("✓ Updating address in config file (preserving other settings)...");
            if let Err(e) = existing_config.write_to_file(&system_config_path) {
                return Err(ServiceError::ConfigError(format!(
                    "Failed to write config: {}",
                    e
                )));
            }
        }
    } else {
        println!("✓ Creating system config file: /etc/bunnylol/config.toml");

        // Create new config with provided ServiceConfig settings
        let mut default_config = BunnylolConfig::default();
        default_config.server.port = config.port;
        default_config.server.address = config.address.clone();
        default_config.server.log_level = config.log_level.clone();

        // Write config file
        if let Err(e) = default_config.write_to_file(&system_config_path) {
            return Err(ServiceError::ConfigError(format!(
                "Failed to write config: {}",
                e
            )));
        }
    }

    println!();

    println!("Service configuration:");
    println!("  Label:       {}", SERVICE_LABEL);
    println!("  Binary:      {}", binary_path.display());
    println!("  Command:     bunnylol serve");
    println!("  Config:      /etc/bunnylol/config.toml");
    println!(
        "    Port:      {} (can be changed in config file)",
        config.port
    );
    println!(
        "    Address:   {} (can be changed in config file)",
        config.address
    );
    println!(
        "    Log level: {} (can be changed in config file)",
        config.log_level
    );
    println!("  Run as:      root");
    println!("  Autostart:   enabled");
    println!();

    let (manager, label) = setup_manager()?;

    let args = vec![OsString::from("serve")];

    let environment = vec![];

    println!("Creating service file...");
    let install_ctx = ServiceInstallCtx {
        label: label.clone(),
        program: binary_path,
        args,
        contents: None,
        username: None,
        working_directory: None,
        environment: Some(environment),
        autostart: true,
        restart_policy: RestartPolicy::OnFailure {
            delay_secs: Some(5),
        },
    };

    manager
        .install(install_ctx)
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service file created and registered");

    println!();
    println!("Starting service...");

    manager
        .start(ServiceStartCtx { label })
        .map_err(|e| ServiceError::ServiceStartFailed(e.to_string()))?;

    println!("✓ Service started");

    println!();
    println!("🎉 Bunnylol server installed successfully!");
    println!();
    println!(
        "Server URL (from config): http://{}:{}",
        config.address, config.port
    );
    println!(
        "Add to browser search: http://{}:{}/?cmd=%s",
        config.address, config.port
    );
    println!();
    println!("To change port/address, edit: /etc/bunnylol/config.toml");
    println!("Then restart the service: sudo bunnylol service restart");

    println!();
    println!("Manage service:");
    println!("  bunnylol service status");
    println!("  bunnylol service logs");
    println!("  bunnylol service restart");
    println!("  bunnylol service uninstall");

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn install_systemd_service(_config: ServiceConfig) -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// Uninstall bunnylol service (Linux only)
#[cfg(target_os = "linux")]
pub fn uninstall_service() -> Result<(), ServiceError> {
    println!("Uninstalling bunnylol system service...");
    println!("Service file: /etc/systemd/system/{}.service", SERVICE_NAME);
    println!();

    let (manager, label) = setup_manager()?;

    println!("Stopping service...");
    let stop_ctx = ServiceStopCtx {
        label: label.clone(),
    };

    match manager.stop(stop_ctx) {
        Ok(_) => println!("✓ Service stopped"),
        Err(e) => {
            let err_msg = e.to_string().to_lowercase();
            if err_msg.contains("not found")
                || err_msg.contains("not loaded")
                || err_msg.contains("could not be found")
            {
                println!("ℹ Service was not running")
            } else {
                println!("⚠ Warning: Could not stop service: {}", e)
            }
        }
    }

    println!("Removing service file...");

    manager
        .uninstall(ServiceUninstallCtx { label })
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service file removed");
    println!();
    println!("✓ Bunnylol service uninstalled successfully");

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn uninstall_service() -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// Start the bunnylol service (Linux only)
#[cfg(target_os = "linux")]
pub fn start_service() -> Result<(), ServiceError> {
    let (manager, label) = setup_manager()?;

    manager
        .start(ServiceStartCtx { label })
        .map_err(|e| ServiceError::ServiceStartFailed(e.to_string()))?;

    println!("✓ Service started");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn start_service() -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// Stop the bunnylol service (Linux only)
#[cfg(target_os = "linux")]
pub fn stop_service() -> Result<(), ServiceError> {
    let (manager, label) = setup_manager()?;

    manager
        .stop(ServiceStopCtx { label })
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service stopped");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn stop_service() -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// Restart the bunnylol service (Linux only)
#[cfg(target_os = "linux")]
pub fn restart_service() -> Result<(), ServiceError> {
    println!("Restarting bunnylol service...");

    let (manager, label) = setup_manager()?;

    manager
        .stop(ServiceStopCtx {
            label: label.clone(),
        })
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;
    println!("✓ Service stopped");

    manager
        .start(ServiceStartCtx { label })
        .map_err(|e| ServiceError::ServiceStartFailed(e.to_string()))?;
    println!("✓ Service started");

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn restart_service() -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// Get the status of the bunnylol service (Linux systemd only)
#[cfg(target_os = "linux")]
pub fn service_status() -> Result<(), ServiceError> {
    let cmd = Command::new("systemctl")
        .args(["status", SERVICE_NAME])
        .status()
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    if !cmd.success() {
        eprintln!(
            "\nNote: Service may not be running (exit code: {})",
            cmd.code().unwrap_or(-1)
        );
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn service_status() -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}

/// View logs for the bunnylol service (Linux systemd only)
#[cfg(target_os = "linux")]
pub fn service_logs(follow: bool, lines: u32) -> Result<(), ServiceError> {
    let mut cmd = Command::new("journalctl");
    cmd.args(["-u", SERVICE_NAME, "-n", &lines.to_string()]);
    if follow {
        cmd.arg("-f");
    }

    let status = cmd
        .status()
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    if !status.success() {
        return Err(ServiceError::ServiceManagerError(format!(
            "journalctl exited with code {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn service_logs(
    #[allow(unused_variables)] follow: bool,
    #[allow(unused_variables)] lines: u32,
) -> Result<(), ServiceError> {
    Err(ServiceError::UnsupportedPlatform)
}
