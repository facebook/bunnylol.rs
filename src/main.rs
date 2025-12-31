/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use clap::{CommandFactory, Parser, Subcommand};

// BunnylolConfig is needed by both server and CLI
use bunnylol::BunnylolConfig;

// CLI-only imports
#[cfg(feature = "cli")]
use bunnylol::{BunnylolCommandRegistry, History, utils};
#[cfg(feature = "cli")]
use clap_complete::generate;
#[cfg(feature = "cli")]
use tabled::{
    Table, Tabled,
    settings::{Color, Modify, Style, Width, object::Columns},
};

#[derive(Parser)]
#[command(name = "bunnylol")]
#[command(
    about = "Smart bookmark server and CLI - URL shortcuts for your browser's search bar and terminal"
)]
#[command(version)]
#[command(override_usage = "bunnylol [OPTIONS] [BINDING] [ARGS]")]
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
        /// Port to bind the server to (overrides config file)
        #[arg(short, long)]
        port: Option<u16>,

        /// Address to bind to (overrides config file)
        #[arg(short, long)]
        address: Option<String>,
    },

    /// List all available command bindings
    #[cfg(feature = "cli")]
    Bindings,

    /// Generate shell completion scripts
    #[cfg(feature = "cli")]
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Manage bunnylol service
    #[cfg(feature = "cli")]
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },

    /// Execute a bunnylol command
    #[cfg(feature = "cli")]
    #[command(external_subcommand)]
    Command(Vec<String>),
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum ServiceAction {
    /// Install bunnylol server as a service (uses config file for port/address)
    Install {
        /// Allow network access (bind to 0.0.0.0). Default: localhost only (127.0.0.1)
        #[arg(short, long)]
        network: bool,
    },
    /// Uninstall bunnylol service
    Uninstall,
    /// Start the server service
    Start,
    /// Stop the server service
    Stop,
    /// Restart the server service
    Restart,
    /// Show server status
    Status,
    /// Show server logs
    Logs {
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
        Some(Commands::Bindings) => {
            print_commands();
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "bunnylol", &mut std::io::stdout());
            Ok(())
        }

        #[cfg(feature = "cli")]
        Some(Commands::Service { action }) => {
            use bunnylol::service::*;

            let result = match action {
                ServiceAction::Install { network } => {
                    // Use ServiceConfig with appropriate address based on --network flag
                    let service_config = ServiceConfig {
                        address: if network {
                            "0.0.0.0".to_string() // Network access
                        } else {
                            "127.0.0.1".to_string() // Localhost only (secure default)
                        },
                        ..Default::default()
                    };

                    install_systemd_service(service_config)
                }
                ServiceAction::Uninstall => uninstall_service(),
                ServiceAction::Start => start_service(),
                ServiceAction::Stop => stop_service(),
                ServiceAction::Restart => restart_service(),
                ServiceAction::Status => service_status(),
                ServiceAction::Logs { follow, lines } => service_logs(follow, lines),
            };

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
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
            let args: Vec<String> = std::env::args()
                .skip(1)
                .filter(|arg| !arg.starts_with('-') && arg != "bunnylol")
                .collect();

            if args.is_empty() {
                // No command provided, print full help
                Cli::command().print_help().unwrap();
                println!(); // Add newline after help
                std::process::exit(0);
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
fn execute_command(
    args: Vec<String>,
    config: &BunnylolConfig,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Special case: "list" should print commands table, not execute as a command
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
    let url =
        BunnylolCommandRegistry::process_command_with_config(command, &resolved_args, Some(config));

    // Print URL
    println!("{}", url);

    // Track command in history if enabled
    if config.history.enabled
        && let Some(history) = History::new(config)
    {
        let username = whoami::username();
        if let Err(e) = history.add(&full_args, &username) {
            eprintln!("Warning: Failed to save command to history: {}", e);
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
        open::that(url)
            .map_err(|e| format!("Failed to open browser: {}. URL printed above.", e))?;
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
        a.bindings[0]
            .to_lowercase()
            .cmp(&b.bindings[0].to_lowercase())
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

    // Get terminal width and calculate column widths dynamically
    let term_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(120); // Default to 120 if terminal size unavailable

    // Use all available width minus 2 for safety
    let available_width = term_width.saturating_sub(2);

    // Calculate widths: Command(15) + Aliases(dynamic) + Description(40-50%) + Example(25-30%)
    let command_width = 15;
    let example_width = (available_width as f32 * 0.25).max(20.0) as usize;
    let description_width = (available_width as f32 * 0.45).max(30.0) as usize;
    let aliases_width = available_width
        .saturating_sub(command_width)
        .saturating_sub(description_width)
        .saturating_sub(example_width);

    let mut table = Table::new(rows);
    table
        .with(Style::rounded())
        .with(Modify::new(Columns::new(0..=0)).with(Color::FG_BRIGHT_CYAN))
        .with(
            Modify::new(Columns::new(1..=1))
                .with(Color::FG_YELLOW)
                .with(Width::wrap(aliases_width)),
        )
        .with(
            Modify::new(Columns::new(2..=2))
                .with(Color::FG_WHITE)
                .with(Width::wrap(description_width)),
        )
        .with(
            Modify::new(Columns::new(3..=3))
                .with(Color::FG_BRIGHT_GREEN)
                .with(Width::wrap(example_width)),
        );

    println!("\n{}\n", table);
    println!("💡 Tip: Use 'bunnylol <command>' to open URLs in your browser");
    println!("   Example: bunnylol ig reels");
    println!("   Use --dry-run to preview the URL without opening it\n");
}
