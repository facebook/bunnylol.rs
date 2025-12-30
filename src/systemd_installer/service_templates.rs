/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// Configuration for service installation
pub struct ServiceConfig {
    pub port: u16,
    pub address: String,
    pub log_level: String,
    pub user_mode: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            address: "0.0.0.0".to_string(),
            log_level: "normal".to_string(),
            user_mode: false,
        }
    }
}

/// Generate systemd service file for system-level installation
pub fn generate_system_service(config: &ServiceConfig) -> String {
    format!(
        r#"[Unit]
Description=Bunnylol Smart Bookmark Server
Documentation=https://github.com/facebook/bunnylol.rs
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
User=bunnylol
Group=bunnylol
ExecStart=/usr/local/bin/bunnylol serve --port {port} --address {address}
Restart=on-failure
RestartSec=5s

# Working directory for config
WorkingDirectory=/var/lib/bunnylol

# Environment
Environment="ROCKET_LOG_LEVEL={log_level}"

# Security hardening (system services)
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/bunnylol

# Additional system-level security
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

# Resource limits
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
"#,
        port = config.port,
        address = config.address,
        log_level = config.log_level,
    )
}

/// Generate systemd service file for user-level installation
pub fn generate_user_service(config: &ServiceConfig) -> String {
    format!(
        r#"[Unit]
Description=Bunnylol Smart Bookmark Server
Documentation=https://github.com/facebook/bunnylol.rs
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
ExecStart=%h/.local/bin/bunnylol serve --port {port} --address {address}
Restart=on-failure
RestartSec=5s

# Environment
Environment="ROCKET_LOG_LEVEL={log_level}"

# Security hardening (user services)
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=%h/.config/bunnylol %h/.local/share/bunnylol %h/.cache/bunnylol

# Resource limits
LimitNOFILE=65536

[Install]
WantedBy=default.target
"#,
        port = config.port,
        address = config.address,
        log_level = config.log_level,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_service_generation() {
        let config = ServiceConfig {
            port: 8000,
            address: "0.0.0.0".to_string(),
            log_level: "normal".to_string(),
            user_mode: false,
        };
        let service = generate_system_service(&config);

        assert!(service.contains("ExecStart=/usr/local/bin/bunnylol serve --port 8000 --address 0.0.0.0"));
        assert!(service.contains("User=bunnylol"));
        assert!(service.contains("WantedBy=multi-user.target"));
        assert!(service.contains("WorkingDirectory=/var/lib/bunnylol"));
    }

    #[test]
    fn test_user_service_generation() {
        let config = ServiceConfig {
            port: 8001,
            address: "127.0.0.1".to_string(),
            log_level: "debug".to_string(),
            user_mode: true,
        };
        let service = generate_user_service(&config);

        assert!(service.contains("ExecStart=%h/.local/bin/bunnylol serve --port 8001 --address 127.0.0.1"));
        assert!(service.contains("WantedBy=default.target"));
        assert!(service.contains("ROCKET_LOG_LEVEL=debug"));
        assert!(!service.contains("User="));
    }
}
