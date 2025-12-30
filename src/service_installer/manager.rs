/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use service_manager::*;
use std::process::Command;
use super::error::InstallError;

const SERVICE_LABEL: &str = "com.facebook.bunnylol";

/// Start the bunnylol service
pub fn start_service(system_mode: bool) -> Result<(), InstallError> {
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    let label: ServiceLabel = SERVICE_LABEL
        .parse()
        .map_err(|e| InstallError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    let service_level = if system_mode {
        ServiceLevel::System
    } else {
        ServiceLevel::User
    };

    manager.set_level(service_level)
        .map_err(|e| InstallError::ServiceManagerError(format!("Failed to set service level: {}", e)))?;

    let start_ctx = ServiceStartCtx {
        label,
    };

    manager
        .start(start_ctx)
        .map_err(|e| InstallError::ServiceStartFailed(e.to_string()))?;

    println!("✓ Service started");
    Ok(())
}

/// Stop the bunnylol service
pub fn stop_service(system_mode: bool) -> Result<(), InstallError> {
    let mut manager = <dyn ServiceManager>::native()
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    let label: ServiceLabel = SERVICE_LABEL
        .parse()
        .map_err(|e| InstallError::ServiceManagerError(format!("Invalid label: {}", e)))?;

    let service_level = if system_mode {
        ServiceLevel::System
    } else {
        ServiceLevel::User
    };

    manager.set_level(service_level)
        .map_err(|e| InstallError::ServiceManagerError(format!("Failed to set service level: {}", e)))?;

    let stop_ctx = ServiceStopCtx {
        label,
    };

    manager
        .stop(stop_ctx)
        .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

    println!("✓ Service stopped");
    Ok(())
}

/// Restart the bunnylol service
pub fn restart_service(system_mode: bool) -> Result<(), InstallError> {
    println!("Restarting bunnylol service...");
    stop_service(system_mode)?;
    start_service(system_mode)?;
    Ok(())
}

/// Get the status of the bunnylol service (platform-specific)
#[allow(unused_variables)]
pub fn service_status(system_mode: bool) -> Result<(), InstallError> {
    // Platform-specific status check
    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("systemctl");
        if !system_mode {
            cmd.arg("--user");
        }
        cmd.args(&["status", "bunnylol"]);

        let status = cmd.status()
            .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

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
            .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

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
pub fn service_logs(system_mode: bool, follow: bool, lines: u32) -> Result<(), InstallError> {
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
            .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

        if !status.success() {
            return Err(InstallError::ServiceManagerError(
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
            .map_err(|e| InstallError::ServiceManagerError(e.to_string()))?;

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

/// Enable autostart (not needed with service-manager as it's set during install)
pub fn enable_service(_system_mode: bool) -> Result<(), InstallError> {
    println!("Service autostart is configured during installation");
    println!("To change autostart, reinstall the service with --autostart flag");
    Ok(())
}

/// Disable autostart (not needed with service-manager)
pub fn disable_service(_system_mode: bool) -> Result<(), InstallError> {
    println!("Service autostart is configured during installation");
    println!("To change autostart, reinstall the service");
    Ok(())
}
