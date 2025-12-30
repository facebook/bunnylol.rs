/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt;

#[derive(Debug)]
pub enum InstallError {
    ServiceManagerError(String),
    BinaryNotFound,
    ServiceStartFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::ServiceManagerError(msg) => {
                write!(f, "service manager error: {}", msg)
            }
            InstallError::BinaryNotFound => {
                write!(f, "bunnylol binary not found in PATH\n\n\
                    Please install bunnylol first:\n  \
                    cargo install bunnylol\n\n\
                    Or install from the current directory:\n  \
                    cargo install --path .")
            }
            InstallError::ServiceStartFailed(msg) => {
                write!(f, "service installed but failed to start: {}", msg)
            }
            InstallError::IoError(e) => {
                write!(f, "I/O error: {}", e)
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
