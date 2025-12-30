/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::ffi::OsString;
use std::fmt;
use std::process::Command;
use service_manager::*;

/// Service label used across all platforms (reverse domain notation)
pub const SERVICE_LABEL: &str = "com.facebook.bunnylol";

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug)]
pub enum ServiceError {
    ServiceManagerError(String),
    BinaryNotFound,
    ServiceStartFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ServiceManagerError(msg) => {
                write!(f, "service manager error: {}", msg)
            }
            ServiceError::BinaryNotFound => {
                write!(f, "bunnylol binary not found in PATH\n\n\
                    Please install bunnylol first:\n  \
                    cargo install bunnylol\n\n\
                    Or install from the current directory:\n  \
                    cargo install --path .")
            }
            ServiceError::ServiceStartFailed(msg) => {
                write!(f, "service installed but failed to start: {}", msg)
            }
            ServiceError::IoError(e) => {
                write!(f, "I/O error: {}", e)
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

// ============================================================================
// Helper Functions
// ============================================================================

/// Common helper to set up service manager with label
fn setup_manager(system_mode: bool) -> Result<(Box<dyn ServiceManager>, ServiceLabel), ServiceError> {
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    let service_level = if system_mode {
        ServiceLevel::System
    } else {
        ServiceLevel::User
    };

    manager.set_level(service_level)
        .map_err(|e| ServiceError::ServiceManagerError(format!("Failed to set service level: {}", e)))?;

    let label: ServiceLabel = SERVICE_LABEL
        .parse()
        .map_err(|e| ServiceError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    Ok((manager, label))
}

// ============================================================================
// Service Lifecycle Functions
// ============================================================================

/// Install bunnylol service using service-manager crate
pub fn install_service(config: ServiceConfig, _force: bool, autostart: bool, start_now: bool) -> Result<(), ServiceError> {
    println!("Installing bunnylol system service...");
    println!();

    // Require binary to be installed and on PATH
    let binary_path = which::which("bunnylol").map_err(|_| ServiceError::BinaryNotFound)?;
    println!("✓ Found bunnylol binary: {}", binary_path.display());

    // Print service file location based on platform
    #[cfg(target_os = "linux")]
    {
        println!("✓ Service file will be created at: /etc/systemd/system/bunnylol.service");
    }

    #[cfg(target_os = "macos")]
    {
        println!("✓ Service file will be created at: /Library/LaunchDaemons/{}.plist", SERVICE_LABEL);
    }

    #[cfg(target_os = "windows")]
    {
        println!("✓ Service will be registered with Windows Service Manager");
    }

    println!();
    println!("Service configuration:");
    println!("  Label:       {}", SERVICE_LABEL);
    println!("  Binary:      {}", binary_path.display());
    println!("  Command:     bunnylol serve --port {} --address {}", config.port, config.address);
    println!("  Port:        {}", config.port);
    println!("  Address:     {}", config.address);
    println!("  Log level:   {}", config.log_level);
    println!("  Autostart:   {}", if autostart { "enabled" } else { "disabled" });
    println!("  Start now:   {}", if start_now { "yes" } else { "no" });
    println!("  Run as:      root");
    println!();

    let (manager, label) = setup_manager(config.system_mode)?;

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

    println!("Creating service file...");
    // Install the service
    let install_ctx = ServiceInstallCtx {
        label: label.clone(),
        program: binary_path,
        args,
        contents: None,  // Use default service file generation
        username: None,  // System services run as root, user services run as current user
        working_directory: None,
        environment: Some(environment),
        autostart,
        restart_policy: RestartPolicy::OnFailure { delay_secs: Some(5) },
    };

    manager
        .install(install_ctx)
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service file created and registered");

    // Start the service if requested
    if start_now {
        println!();
        println!("Starting service...");
        let start_ctx = ServiceStartCtx {
            label: label.clone(),
        };

        manager
            .start(start_ctx)
            .map_err(|e| ServiceError::ServiceStartFailed(e.to_string()))?;

        println!("✓ Service started");

        // Wait a bit for startup
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("✓ Service appears to be running");
    }

    println!();
    println!("🎉 Bunnylol server installed successfully!");
    println!();
    println!("Server URL: http://{}:{}", config.address, config.port);
    println!("Add to browser search: http://{}:{}/?cmd=%s", config.address, config.port);

    println!();
    println!("Manage service:");
    println!("  bunnylol service status");
    println!("  bunnylol service logs");
    println!("  bunnylol service restart");
    println!("  bunnylol service uninstall");

    Ok(())
}

/// Uninstall bunnylol service
pub fn uninstall_service(system_mode: bool) -> Result<(), ServiceError> {
    println!("Uninstalling bunnylol system service...");
    println!();

    // Print service file location based on platform
    #[cfg(target_os = "linux")]
    {
        println!("Service file: /etc/systemd/system/bunnylol.service");
    }

    #[cfg(target_os = "macos")]
    {
        println!("Service file: /Library/LaunchDaemons/{}.plist", SERVICE_LABEL);
    }

    #[cfg(target_os = "windows")]
    {
        println!("Unregistering from Windows Service Manager");
    }

    println!();

    let (manager, label) = setup_manager(system_mode)?;

    // Stop the service first (ignore errors if already stopped)
    println!("Stopping service...");
    let stop_ctx = ServiceStopCtx {
        label: label.clone(),
    };

    match manager.stop(stop_ctx) {
        Ok(_) => println!("✓ Service stopped"),
        Err(_) => println!("ℹ Service was not running"),
    }

    // Uninstall the service
    println!("Removing service file...");
    let uninstall_ctx = ServiceUninstallCtx {
        label: label.clone(),
    };

    manager
        .uninstall(uninstall_ctx)
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service file removed");
    println!();
    println!("✓ Bunnylol service uninstalled successfully");

    Ok(())
}

/// Start the bunnylol service
pub fn start_service(system_mode: bool) -> Result<(), ServiceError> {
    let (manager, label) = setup_manager(system_mode)?;

    let start_ctx = ServiceStartCtx {
        label,
    };

    manager
        .start(start_ctx)
        .map_err(|e| ServiceError::ServiceStartFailed(e.to_string()))?;

    println!("✓ Service started");
    Ok(())
}

/// Stop the bunnylol service
pub fn stop_service(system_mode: bool) -> Result<(), ServiceError> {
    let (manager, label) = setup_manager(system_mode)?;

    let stop_ctx = ServiceStopCtx {
        label,
    };

    manager
        .stop(stop_ctx)
        .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service stopped");
    Ok(())
}

/// Restart the bunnylol service
pub fn restart_service(system_mode: bool) -> Result<(), ServiceError> {
    println!("Restarting bunnylol service...");
    stop_service(system_mode)?;
    start_service(system_mode)?;
    Ok(())
}

/// Get the status of the bunnylol service (platform-specific)
#[allow(unused_variables)]
pub fn service_status(system_mode: bool) -> Result<(), ServiceError> {
    // Platform-specific status check
    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("systemctl");
        if !system_mode {
            cmd.arg("--user");
        }
        cmd.args(&["status", "bunnylol"]);

        let status = cmd.status()
            .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

        if !status.success() {
            eprintln!("\nNote: Service may not be running (exit code: {})", status.code().unwrap_or(-1));
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        let status = Command::new("launchctl")
            .args(&["list", SERVICE_LABEL])
            .status()
            .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

        if !status.success() {
            eprintln!("\nNote: Service may not be running");
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        eprintln!("Status checking not implemented for this platform");
        Ok(())
    }
}

/// View logs for the bunnylol service (platform-specific)
#[allow(unused_variables)]
pub fn service_logs(system_mode: bool, follow: bool, lines: u32) -> Result<(), ServiceError> {
    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("journalctl");
        if !system_mode {
            cmd.arg("--user");
        }
        cmd.args(&["-u", "bunnylol", "-n", &lines.to_string()]);
        if follow {
            cmd.arg("-f");
        }

        let status = cmd.status()
            .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

        if !status.success() {
            return Err(ServiceError::ServiceManagerError(
                format!("journalctl exited with code {}", status.code().unwrap_or(-1))
            ));
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        // On macOS, logs go to system.log or can be viewed with `log show`
        let mut cmd = Command::new("log");
        cmd.args(&["show", "--predicate", &format!("processImagePath CONTAINS \"bunnylol\""), "--last", &format!("{}m", lines)]);

        if follow {
            cmd.arg("--style").arg("syslog");
        }

        let status = cmd.status()
            .map_err(|e| ServiceError::ServiceManagerError(e.to_string()))?;

        if !status.success() {
            eprintln!("Note: Could not retrieve logs. You can also check Console.app");
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        eprintln!("Log viewing not implemented for this platform");
        Ok(())
    }
}
