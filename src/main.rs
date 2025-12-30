/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
use bunnylol::{BunnylolCommandRegistry, BunnylolConfig, History, utils};
#[cfg(feature = "cli")]
use tabled::{
    Table, Tabled,
    settings::{Style, Color, Modify, object::Columns},
};

#[derive(Parser)]
#[command(name = "bunnylol")]
#[command(about = "Smart bookmark server and CLI - URL shortcuts for your browser's search bar and terminal")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print URL without opening browser (for command execution mode)
    #[arg(short = 'n', long, global = true)]
    dry_run: bool,

    /// List all available commands
    #[arg(short, long, global = true)]
    list: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the bunnylol web server
    #[cfg(feature = "server")]
    Serve {
        /// Port to bind the server to (overrides config)
        #[arg(short, long)]
        port: Option<u16>,

        /// Address to bind to (overrides config)
        #[arg(short, long)]
        address: Option<String>,
    },

    /// Install bunnylol server as a service
    #[cfg(feature = "cli")]
    InstallServer {
        /// Install as system-level service (default is user-level)
        #[arg(long)]
        system: bool,

        /// Port to bind the server to
        #[arg(long, default_value = "8000")]
        port: u16,

        /// Address to bind to
        #[arg(long)]
        address: Option<String>,

        /// Overwrite existing service
        #[arg(long)]
        force: bool,

        /// Disable autostart on boot
        #[arg(long)]
        no_autostart: bool,

        /// Do not start service after installation
        #[arg(long)]
        no_start: bool,
    },

    /// Uninstall bunnylol service
    #[cfg(feature = "cli")]
    UninstallServer {
        /// Uninstall system-level service
        #[arg(long)]
        system: bool,
    },

    /// Manage bunnylol service
    #[cfg(feature = "cli")]
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },

    /// Execute a bunnylol command
    #[cfg(feature = "cli")]
    #[command(external_subcommand)]
    Command(Vec<String>),
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum ServerAction {
    /// Start the server service
    Start {
        #[arg(long)]
        system: bool,
    },
    /// Stop the server service
    Stop {
        #[arg(long)]
        system: bool,
    },
    /// Restart the server service
    Restart {
        #[arg(long)]
        system: bool,
    },
    /// Show server status
    Status {
        #[arg(long)]
        system: bool,
    },
    /// Show server logs
    Logs {
        #[arg(long)]
        system: bool,
        #[arg(short, long)]
        follow: bool,
        #[arg(short = 'n', long, default_value = "20")]
        lines: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration
    let config = match BunnylolConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Warning: {}", e);
            eprintln!("Continuing with default configuration...");
            BunnylolConfig::default()
        }
    };

    // Handle global --list flag
    #[cfg(feature = "cli")]
    if cli.list {
        print_commands();
        return Ok(());
    }

    match cli.command {
        #[cfg(feature = "server")]
        Some(Commands::Serve { port, address }) => {
            // Override config with command-line arguments if provided
            let mut server_config = config.clone();
            if let Some(p) = port {
                server_config.server.port = p;
            }
            if let Some(a) = address {
                server_config.server.address = a;
            }

            // Launch the server
            bunnylol::server::launch(server_config).await?;
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::InstallServer { system, port, address, force, no_autostart, no_start }) => {
            use bunnylol::service::{ServiceConfig, install_service};

            let service_config = ServiceConfig {
                port,
                address: address.unwrap_or_else(|| {
                    if system { "0.0.0.0".to_string() } else { "127.0.0.1".to_string() }
                }),
                log_level: config.server.log_level.clone(),
                system_mode: system,
            };

            install_service(service_config, force, !no_autostart, !no_start)?;
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::UninstallServer { system }) => {
            use bunnylol::service::uninstall_service;
            uninstall_service(system)?;
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::Server { action }) => {
            use bunnylol::service::*;

            match action {
                ServerAction::Start { system } => start_service(system)?,
                ServerAction::Stop { system } => stop_service(system)?,
                ServerAction::Restart { system } => restart_service(system)?,
                ServerAction::Status { system } => service_status(system)?,
                ServerAction::Logs { system, follow, lines } => service_logs(system, follow, lines)?,
            }
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::Command(args)) => {
            execute_command(args, &config, cli.dry_run)?;
            Ok(())
        }

        // No subcommand provided - treat remaining args as a command to execute
        #[cfg(feature = "cli")]
        None => {
            // Check if there are any remaining arguments (passed as positional)
            let args: Vec<String> = std::env::args().skip(1)
                .filter(|arg| !arg.starts_with('-') && arg != "bunnylol")
                .collect();

            if args.is_empty() {
                // No command provided, print help
                eprintln!("Error: No command provided\n");
                eprintln!("Usage: bunnylol [OPTIONS] [COMMAND]\n");
                eprintln!("Run 'bunnylol --help' for more information");
                std::process::exit(1);
            }

            execute_command(args, &config, cli.dry_run)?;
            Ok(())
        }

        #[cfg(not(feature = "cli"))]
        None => {
            eprintln!("Error: No command provided. This binary was built without CLI support.");
            eprintln!("Use 'bunnylol serve' to run the server, or rebuild with --features cli");
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "cli")]
fn execute_command(args: Vec<String>, config: &BunnylolConfig, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if first arg is "list"
    if args.first().map(|s| s.as_str()) == Some("list") {
        print_commands();
        return Ok(());
    }

    // Join command parts (e.g., ["ig", "reels"] -> "ig reels")
    let full_args = args.join(" ");

    // Resolve command aliases
    let resolved_args = config.resolve_command(&full_args);

    // Extract command and process with config for custom search engine
    let command = utils::get_command_from_query_string(&resolved_args);
    let url = BunnylolCommandRegistry::process_command_with_config(command, &resolved_args, Some(config));

    // Print URL
    println!("{}", url);

    // Track command in history if enabled
    if config.history.enabled {
        if let Some(history) = History::new(config) {
            let username = whoami::username();
            if let Err(e) = history.add(&full_args, &username) {
                eprintln!("Warning: Failed to save command to history: {}", e);
            }
        }
    }

    // Open in browser unless --dry-run
    if !dry_run {
        open_url(&url, config)?;
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn open_url(url: &str, config: &BunnylolConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(browser) = &config.browser {
        // Open with specified browser
        open::with(url, browser).map_err(|e| {
            format!(
                "Failed to open browser '{}': {}. URL printed above.",
                browser, e
            )
        })?;
    } else {
        // Use system default browser
        open::that(url).map_err(|e| {
            format!("Failed to open browser: {}. URL printed above.", e)
        })?;
    }
    Ok(())
}

#[cfg(feature = "cli")]
#[derive(Tabled)]
struct CommandRow {
    #[tabled(rename = "Command")]
    command: String,
    #[tabled(rename = "Aliases")]
    aliases: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Example")]
    example: String,
}

#[cfg(feature = "cli")]
fn print_commands() {
    let mut commands = BunnylolCommandRegistry::get_all_commands().clone();
    commands.sort_by(|a, b| {
        a.bindings[0].to_lowercase().cmp(&b.bindings[0].to_lowercase())
    });

    let rows: Vec<CommandRow> = commands
        .into_iter()
        .map(|cmd| {
            let primary = cmd.bindings.first().unwrap_or(&String::new()).clone();
            let aliases = if cmd.bindings.len() > 1 {
                cmd.bindings[1..].join(", ")
            } else {
                String::from("—")
            };

            CommandRow {
                command: primary,
                aliases,
                description: cmd.description,
                example: cmd.example,
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table
        .with(Style::rounded())
        .with(Modify::new(Columns::single(0)).with(Color::FG_BRIGHT_CYAN))
        .with(Modify::new(Columns::single(1)).with(Color::FG_YELLOW))
        .with(Modify::new(Columns::single(2)).with(Color::FG_WHITE))
        .with(Modify::new(Columns::single(3)).with(Color::FG_BRIGHT_GREEN));

    println!("\n{}\n", table);
    println!("💡 Tip: Use 'bunnylol <command>' to open URLs in your browser");
    println!("   Example: bunnylol ig reels");
    println!("   Use --dry-run to preview the URL without opening it\n");
}
