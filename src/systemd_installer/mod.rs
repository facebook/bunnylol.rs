/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod error;
pub mod installer;
pub mod manager;
pub mod service_templates;

pub use error::InstallError;
pub use installer::{install_service, uninstall_service};
pub use manager::{start_service, stop_service, restart_service, enable_service, disable_service, service_status, service_logs};
