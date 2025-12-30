/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use clap::Parser;
use bunnylol::{BunnylolCommandRegistry, utils};
use tabled::{
    Table, Tabled,
    settings::{Style, Color, Modify, object::Columns},
};

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

#[derive(Parser)]
#[command(name = "bunnylol")]
#[command(about = "Smart bookmark CLI - Open URLs from your terminal")]
#[command(version)]
struct Cli {
    /// Command to execute (e.g., "ig reels", "gh facebook/react")
    #[arg(trailing_var_arg = true, required_unless_present = "list")]
    command: Vec<String>,

    /// Print URL without opening browser
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// List all available commands
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Check if --list flag is set OR if first command is "list"
    let should_list = cli.list || cli.command.first().map(|s| s.as_str()) == Some("list");

    if should_list {
        print_commands();
        return Ok(());
    }

    // Join command parts (e.g., ["ig", "reels"] -> "ig reels")
    let full_args = cli.command.join(" ");

    // Extract command and process
    let command = utils::get_command_from_query_string(&full_args);
    let url = BunnylolCommandRegistry::process_command(command, &full_args);

    // Print URL
    println!("{}", url);

    // Open in browser unless --dry-run
    if !cli.dry_run {
        open::that(&url).map_err(|e| {
            format!("Failed to open browser: {}. URL printed above.", e)
        })?;
    }

    Ok(())
}

fn print_commands() {
    let mut commands = BunnylolCommandRegistry::get_all_commands();
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
    println!("💡 Tip: Use 'bunnylol-cli <command>' to open URLs in your browser");
    println!("   Example: bunnylol-cli ig reels");
    println!("   Use --dry-run to preview the URL without opening it\n");
}
