/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::process::Command;
use super::error::InstallError;

/// Start the bunnylol service
pub fn start_service(user_mode: bool) -> Result<(), InstallError> {
    let output = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "start", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["start", "bunnylol"])
            .output()
    };

    let result = output.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if result.status.success() {
        println!("✓ Service started");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(InstallError::ServiceStartFailed(stderr.to_string()))
    }
}

/// Stop the bunnylol service
pub fn stop_service(user_mode: bool) -> Result<(), InstallError> {
    let output = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "stop", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["stop", "bunnylol"])
            .output()
    };

    let result = output.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if result.status.success() {
        println!("✓ Service stopped");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(InstallError::CommandFailed(stderr.to_string()))
    }
}

/// Restart the bunnylol service
pub fn restart_service(user_mode: bool) -> Result<(), InstallError> {
    println!("Restarting bunnylol service...");
    let output = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "restart", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["restart", "bunnylol"])
            .output()
    };

    let result = output.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if result.status.success() {
        println!("✓ Service restarted");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(InstallError::ServiceStartFailed(stderr.to_string()))
    }
}

/// Enable the bunnylol service to start on boot
pub fn enable_service(user_mode: bool) -> Result<(), InstallError> {
    let output = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "enable", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["enable", "bunnylol"])
            .output()
    };

    let result = output.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if result.status.success() {
        println!("✓ Service enabled");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(InstallError::CommandFailed(stderr.to_string()))
    }
}

/// Disable the bunnylol service from starting on boot
pub fn disable_service(user_mode: bool) -> Result<(), InstallError> {
    let output = if user_mode {
        Command::new("systemctl")
            .args(&["--user", "disable", "bunnylol"])
            .output()
    } else {
        Command::new("systemctl")
            .args(&["disable", "bunnylol"])
            .output()
    };

    let result = output.map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if result.status.success() {
        println!("✓ Service disabled");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        Err(InstallError::CommandFailed(stderr.to_string()))
    }
}

/// Get the status of the bunnylol service
pub fn service_status(user_mode: bool) -> Result<(), InstallError> {
    let mut cmd = Command::new("systemctl");

    if user_mode {
        cmd.arg("--user");
    }

    cmd.args(&["status", "bunnylol"]);

    let status = cmd.status()
        .map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    // systemctl status exits with 0 if running, non-zero otherwise
    // But we still want to show the status either way
    if !status.success() {
        eprintln!("\nNote: Service may not be running (exit code: {})", status.code().unwrap_or(-1));
    }

    Ok(())
}

/// View logs for the bunnylol service
pub fn service_logs(user_mode: bool, follow: bool, lines: u32) -> Result<(), InstallError> {
    let mut cmd = Command::new("journalctl");

    if user_mode {
        cmd.arg("--user");
    }

    cmd.args(&["-u", "bunnylol", "-n", &lines.to_string()]);

    if follow {
        cmd.arg("-f");
    }

    let status = cmd.status()
        .map_err(|e| InstallError::CommandFailed(e.to_string()))?;

    if !status.success() {
        return Err(InstallError::CommandFailed(
            format!("journalctl exited with code {}", status.code().unwrap_or(-1))
        ));
    }

    Ok(())
}
