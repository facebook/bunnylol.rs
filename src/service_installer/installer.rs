/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use service_manager::*;

use super::error::InstallError;

/// Configuration for service installation
pub struct ServiceConfig {
    pub port: u16,
    pub address: String,
    pub log_level: String,
    pub system_mode: bool,  // true = system-level, false = user-level
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            address: "0.0.0.0".to_string(),
            log_level: "normal".to_string(),
            system_mode: true,
        }
    }
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
    let release_path = std::path::Path::new("target/release/bunnylol");
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

    let binary_path = std::path::Path::new("target/release/bunnylol");
    if binary_path.exists() {
        Ok(binary_path.to_path_buf())
    } else {
        Err(InstallError::BinaryNotFound)
    }
}

/// Install bunnylol service using service-manager crate
pub fn install_service(config: ServiceConfig, _force: bool, autostart: bool, start_now: bool) -> Result<(), InstallError> {
    println!("Installing bunnylol service...");

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

    // Get the native service manager for this platform
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    // Set service level (user vs system)
    let service_level = if config.system_mode {
        ServiceLevel::System
    } else {
        ServiceLevel::User
    };

    manager.set_level(service_level)
        .map_err(|e| InstallError::ServiceManagerError(format!("Failed to set service level: {}", e)))?;

    // Create service label (reverse domain notation)
    let label: ServiceLabel = "rs.bunnylol.server"
        .parse()
        .map_err(|e| InstallError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    // Prepare arguments for the bunnylol serve command
    let args = vec![
        OsString::from("serve"),
        OsString::from("--port"),
        OsString::from(config.port.to_string()),
        OsString::from("--address"),
        OsString::from(&config.address),
    ];

    // Add environment variables for Rocket
    let environment = vec![
        (
            "ROCKET_LOG_LEVEL".to_string(),
            config.log_level.clone(),
        ),
    ];

    // Install the service
    let install_ctx = ServiceInstallCtx {
        label: label.clone(),
        program: binary_path,
        args,
        contents: None,  // Use default service file generation
        username: if config.system_mode {
            Some("bunnylol".to_string())
        } else {
            None
        },
        working_directory: None,
        environment: Some(environment),
        autostart,
        restart_policy: RestartPolicy::OnFailure { delay_secs: Some(5) },
    };

    manager
        .install(install_ctx)
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service installed");

    // Start the service if requested
    if start_now {
        let start_ctx = ServiceStartCtx {
            label: label.clone(),
        };

        manager
            .start(start_ctx)
            .map_err(|e| InstallError::ServiceStartFailed(e.to_string()))?;

        println!("✓ Service started");

        // Wait a bit for startup
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("✓ Service appears to be running");
    }

    println!("\n🎉 Bunnylol server installed successfully!");
    println!("\nServer URL: http://{}:{}", config.address, config.port);
    println!("Add to browser search: http://{}:{}/?cmd=%s", config.address, config.port);

    println!("\nManage service:");
    println!("  bunnylol server status{}", if config.system_mode { " --system" } else { "" });
    println!("  bunnylol server logs{}", if config.system_mode { " --system" } else { "" });
    println!("  bunnylol server restart{}", if config.system_mode { " --system" } else { "" });

    Ok(())
}

/// Uninstall bunnylol service
pub fn uninstall_service(system_mode: bool) -> Result<(), InstallError> {
    println!("Uninstalling bunnylol service...");

    // Get the native service manager
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    // Set service level
    let service_level = if system_mode {
        ServiceLevel::System
    } else {
        ServiceLevel::User
    };

    manager.set_level(service_level)
        .map_err(|e| InstallError::ServiceManagerError(format!("Failed to set service level: {}", e)))?;

    // Create service label
    let label: ServiceLabel = "rs.bunnylol.server"
        .parse()
        .map_err(|e| InstallError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    // Stop the service first (ignore errors if already stopped)
    let stop_ctx = ServiceStopCtx {
        label: label.clone(),
    };

    let _ = manager.stop(stop_ctx);
    println!("✓ Service stopped");

    // Uninstall the service
    let uninstall_ctx = ServiceUninstallCtx {
        label: label.clone(),
    };

    manager
        .uninstall(uninstall_ctx)
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service uninstalled");
    println!("\n✓ Bunnylol service removed successfully");

    Ok(())
}
