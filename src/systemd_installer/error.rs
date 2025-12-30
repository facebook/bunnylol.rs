/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt;

#[derive(Debug)]
pub enum InstallError {
    SystemdNotAvailable,
    UserSystemdNotRunning,
    AlreadyInstalled,
    PermissionDenied,
    BinaryNotFound,
    BuildFailed(String),
    PortInUse(u16),
    ServiceStartFailed(String),
    ServiceNotInstalled,
    HealthCheckFailed,
    UserCreationFailed(String),
    IoError(std::io::Error),
    CommandFailed(String),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::SystemdNotAvailable => {
                write!(f, "systemd is not available on this system")
            }
            InstallError::UserSystemdNotRunning => {
                write!(f, "systemd --user is not running. Try: systemctl --user status")
            }
            InstallError::AlreadyInstalled => {
                write!(f, "bunnylol service is already installed. Use --force to overwrite, or uninstall first.")
            }
            InstallError::PermissionDenied => {
                write!(f, "permission denied. System-level installation requires sudo.")
            }
            InstallError::BinaryNotFound => {
                write!(f, "bunnylol binary not found in PATH or target/release/")
            }
            InstallError::BuildFailed(msg) => {
                write!(f, "failed to build bunnylol: {}", msg)
            }
            InstallError::PortInUse(port) => {
                write!(f, "port {} is already in use", port)
            }
            InstallError::ServiceStartFailed(msg) => {
                write!(f, "service installed but failed to start: {}", msg)
            }
            InstallError::ServiceNotInstalled => {
                write!(f, "bunnylol service is not installed")
            }
            InstallError::HealthCheckFailed => {
                write!(f, "service started but health check failed")
            }
            InstallError::UserCreationFailed(msg) => {
                write!(f, "failed to create system user 'bunnylol': {}", msg)
            }
            InstallError::IoError(e) => {
                write!(f, "I/O error: {}", e)
            }
            InstallError::CommandFailed(msg) => {
                write!(f, "command failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for InstallError {}

impl From<std::io::Error> for InstallError {
    fn from(err: std::io::Error) -> Self {
        InstallError::IoError(err)
    }
}
