/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static PROMETHEUS_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Initialize the Prometheus metrics exporter
/// This should be called once during server startup
pub fn init_metrics() {
    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus exporter");

    if PROMETHEUS_HANDLE.set(handle).is_err() {
        eprintln!("Warning: Metrics already initialized");
    }
}

/// Get the current Prometheus metrics as a string
pub fn get_metrics() -> String {
    PROMETHEUS_HANDLE
        .get()
        .map(|handle| handle.render())
        .unwrap_or_else(|| "# Metrics not initialized\n".to_string())
}

/// Track a redirect request
pub fn track_request(command: &str, success: bool) {
    metrics::counter!("bunnylol_requests_total", "command" => command.to_string(), "status" => if success { "success" } else { "error" }).increment(1);
}

/// Track request duration
pub fn track_request_duration(command: &str, duration_ms: f64) {
    metrics::histogram!("bunnylol_request_duration_milliseconds", "command" => command.to_string())
        .record(duration_ms);
}

/// Increment active requests counter
pub fn increment_active_requests() {
    metrics::gauge!("bunnylol_active_requests").increment(1.0);
}

/// Decrement active requests counter
pub fn decrement_active_requests() {
    metrics::gauge!("bunnylol_active_requests").decrement(1.0);
}

/// Track command usage
pub fn track_command_usage(command: &str) {
    metrics::counter!("bunnylol_command_usage_total", "command" => command.to_string())
        .increment(1);
}
