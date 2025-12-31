# CLAUDE.md - Developer Guide for AI Assistants

This guide provides context about the bunnylol.rs repository structure and patterns to help work efficiently on this codebase.

## Project Overview

**bunnylol.rs** is a smart bookmark server written in Rust that lets you create URL shortcuts accessible from your browser's search bar. It's a modern Rust implementation of [bunny1](https://github.com/ccheever/bunny1).

**Tech Stack:**
- **Language:** Rust (2024 edition)
- **Web Framework:** Rocket 0.5 (async)
- **Frontend:** Leptos 0.6 (SSR for bindings page)
- **CLI:** clap 4.5 with subcommands
- **Deployment:** Native services (systemd/launchd/Windows Service) or Docker (compose v2)

**Key Features:**
- Smart URL routing with command patterns (e.g., `gh username/repo` → GitHub)
- Multiple aliases per command (e.g., `ig`/`instagram`, `tw`/`twitter`)
- Subcommand support (e.g., `meta pay`, `ig reels`)
- Default Google search fallback
- Web portal to view all command bindings
- Unified CLI with command execution and server management

## Repository Structure

```
bunnylol.rs/
├── src/
│   ├── main.rs                          # CLI entry point and dispatcher
│   ├── lib.rs                           # Library exports
│   ├── config.rs                        # Configuration (server, aliases, history)
│   ├── server/
│   │   ├── mod.rs                       # Rocket server setup and routing
│   │   ├── routes.rs                    # HTTP route handlers
│   │   └── web.rs                       # Web response helpers
│   ├── commands/
│   │   ├── mod.rs                       # Module exports
│   │   ├── github.rs                    # Example: gh command
│   │   ├── instagram.rs                 # Example: ig command with subcommands
│   │   ├── meta.rs                      # Example: meta command with subcommands
│   │   └── [30+ other command files]
│   ├── utils/
│   │   ├── bunnylol_command.rs          # Core trait & registry
│   │   └── url_encoding.rs              # URL building helpers
│   ├── components/
│   │   └── bindings_page.rs             # Leptos UI for /bindings
│   └── service_installer/               # Cross-platform service installation
│       ├── mod.rs
│       ├── installer.rs                 # Install/uninstall services
│       ├── manager.rs                   # Service management (start/stop/logs)
│       └── error.rs                     # Error types
├── Cargo.toml
├── docker-compose.yml
├── Dockerfile
├── README.md
└── CLAUDE.md (this file)
```

## Architecture Patterns

### 1. BunnylolCommand Trait

All commands implement the `BunnylolCommand` trait defined in `src/utils/bunnylol_command.rs`:

```rust
pub trait BunnylolCommand {
    const BINDINGS: &'static [&'static str];  // Command aliases
    fn process_args(args: &str) -> String;     // Returns URL
    fn get_info() -> CommandInfo;              // For documentation
}
```

### 2. Command Registration

Commands are registered in two places:

1. **`src/commands/mod.rs`** - Module exports:
   ```rust
   pub use self::github::GitHubCommand;
   pub use self::instagram::InstagramCommand;
   // ... etc
   ```

2. **`src/utils/bunnylol_command.rs`** - In `BunnylolCommandRegistry`:
   - `process_command()` method (~line 74-108): Routes commands to handlers
   - `get_all_commands()` method (~line 112-148): Lists all commands for /bindings page

### 3. URL Building Helpers

Located in `src/utils/url_encoding.rs`:
- `build_search_url(base, param, query)` - Constructs search URLs with encoded params
- `build_path_url(base, path)` - Appends path to base URL

## How to Add New Commands

### Adding a Brand New Command

1. **Create command file** in `src/commands/your_command.rs`:
   ```rust
   use crate::utils::bunnylol_command::{BunnylolCommand, CommandInfo};

   pub struct YourCommand;

   impl BunnylolCommand for YourCommand {
       const BINDINGS: &'static [&'static str] = &["alias1", "alias2"];

       fn process_args(args: &str) -> String {
           let query = Self::get_command_args(args);
           // Return URL based on query
           "https://example.com".to_string()
       }

       fn get_info() -> CommandInfo {
           CommandInfo {
               bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
               description: "Description here".to_string(),
               example: "alias1 example".to_string(),
           }
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_your_command() {
           assert_eq!(YourCommand::process_args("alias1"), "https://example.com");
       }
   }
   ```

2. **Export in `src/commands/mod.rs`**:
   ```rust
   pub mod your_command;
   pub use self::your_command::YourCommand;
   ```

3. **Register in `src/bunnylol_command_registry.rs`** - Add to the `register_commands!` macro:
   ```rust
   register_commands! {
       crate::commands::BindingsCommand,
       // ... other commands ...
       crate::commands::YourCommand,  // ADD YOUR COMMAND HERE
   }
   ```

   **IMPORTANT:** The `register_commands!` macro automatically generates both:
   - `initialize_command_lookup()` - Maps aliases to handlers
   - `get_all_commands_impl()` - Lists all commands for /bindings page

   You only need to add your command once to the macro, and it will be registered everywhere.

### Adding Subcommands to Existing Commands

**Much simpler!** Just edit the existing command file:

1. **Update the `process_args` method** with a match statement:
   ```rust
   fn process_args(args: &str) -> String {
       let query = Self::get_command_args(args);
       match query {
           "subcommand1" => "https://example.com/sub1".to_string(),
           "sub2" | "alias2" => "https://example.com/sub2".to_string(),  // Multiple aliases
           _ => "https://example.com".to_string(),  // Default
       }
   }
   ```

2. **Add tests** for the new subcommands

3. **Update doc comment** at top of file

**No registration needed** - the command is already hooked up!

**Example:** See `src/commands/instagram.rs` for `reels`, `messages`, `msg`, `chat` subcommands, or `src/commands/meta.rs` for `pay`, `accounts`, `ai` subcommands.

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific command
cargo test instagram
cargo test meta

# Run with output
cargo test -- --nocapture
```

### Test Patterns

All commands include unit tests in `#[cfg(test)]` modules:
- Test base command (no args)
- Test each alias
- Test subcommands
- Test search/dynamic behavior
- Test edge cases

**Example test:**
```rust
#[test]
fn test_instagram_command_reels() {
    assert_eq!(
        InstagramCommand::process_args("ig reels"),
        "https://www.instagram.com/reels/"
    );
}
```

## Building and Running

```bash
# Development
cargo run -- serve            # Starts server on localhost:8000
cargo run -- gh facebook/react # Execute a command
cargo build                   # Build without running
cargo check                   # Fast syntax check

# Docker
docker compose up -d          # Run on port 8000
BUNNYLOL_PORT=9000 docker compose up  # Custom port

# Testing
cargo test                    # Run all tests
cargo test --test ''          # (Don't use - this errors)

# Service Installation (cross-platform: Linux/macOS/Windows)
cargo install --path .
sudo bunnylol install-server --system  # System-level service
bunnylol install-server                # User-level service
sudo bunnylol server status --system   # Check status
```

## Key Implementation Details

### Command Resolution Flow

1. User types: `http://localhost:8000/?cmd=ig reels`
2. Rocket routes to main handler
3. `BunnylolCommandRegistry::process_command()` extracts command: `"ig"`
4. Registry matches `"ig"` to `InstagramCommand`
5. `InstagramCommand::process_args("ig reels")` is called
6. `get_command_args()` strips `"ig"` prefix → `"reels"`
7. Command returns `"https://www.instagram.com/reels/"`
8. Server sends 302 redirect

### Multiple Alias Pattern

```rust
const BINDINGS: &'static [&'static str] = &["alias1", "alias2"];
```

The `matches_command()` trait method automatically checks all bindings.

### Subcommand Pattern with Match

```rust
match query {
    "sub1" => "url1",
    "sub2" | "sub2_alias" => "url2",  // Multiple aliases for one subcommand
    "" => "default_url",               // No args
    _ => {                             // Fallback (search, etc.)
        // Handle dynamic args
    }
}
```

### Special Patterns

- **Prefix commands:** Dollar sign (`$AAPL`) handled specially in `process_prefix_commands()`
- **Default search:** Any unmatched command falls through to Google search
- **Profile syntax:** `@username` pattern (see Twitter, Instagram, Threads commands)
- **Subreddit syntax:** `r/subreddit` pattern (see Reddit command)

## Common Tasks Reference

### View all available commands
Navigate to `http://localhost:8000/?cmd=bindings` (or use aliases: `commands`, `list`)

### Add a simple redirect
Edit existing command or create new one with static URL return

### Add search functionality
Use `build_search_url()` helper from `url_encoding.rs`

### Add profile lookup
Parse args for `@` prefix (see `instagram.rs`, `twitter.rs`, `threads.rs`)

### Support special syntax
Add parsing logic in `process_args()` (see `reddit.rs` for `r/` pattern)

## Tips for Efficient Development

1. **Use the Explore agent** when you need to understand existing patterns or find similar commands
2. **Read existing commands** for patterns before creating new ones (Instagram, Meta, YouTube are good examples)
3. **Always add tests** - the project has comprehensive test coverage
4. **Follow the existing patterns** - consistency is valued over creativity here
5. **Don't modify registration** when adding subcommands to existing commands
6. **Use parallel tool calls** when reading multiple command files for context
7. **Check `url_encoding.rs`** before writing custom URL builders

## Recent Changes

- 2025-12-30: **Major refactor** - Merged binaries, added cross-platform service installation
  - Unified `bunnylol-server` and `bunnylol-cli` into single `bunnylol` binary
  - Server now runs with `bunnylol serve` subcommand
  - Added cross-platform service installation (systemd/launchd/Windows Service)
  - New service management commands: `install-server`, `server start/stop/status/logs`, etc.
  - Moved server code to `src/server/` module
  - Added `ServerConfig` to centralize server configuration
- 2025-12-29: Added `meta pay`, `ig reels`, `ig messages/msg/chat` subcommands
- See git log for full history: `git log --oneline`

## Troubleshooting

**Tests failing?**
- Check URL formatting (trailing slashes, query params)
- Verify match arm order (specific before general)
- Ensure test name doesn't conflict with existing tests

**Command not working?**
- Verify registration in `process_command()` match statement
- Check command is exported in `mod.rs`
- Ensure BINDINGS array is correct
- Test with `cargo test your_command`

**Build errors?**
- Run `cargo check` for fast feedback
- Check imports at top of file
- Verify trait implementation is complete

## Reference Commands

**Best examples to study:**
- `src/commands/instagram.rs` - Profile lookup, search, subcommands
- `src/commands/meta.rs` - Multiple subcommands, special binding behavior
- `src/commands/youtube.rs` - Complex subcommand routing
- `src/commands/github.rs` - Path parsing for usernames/repos
- `src/commands/reddit.rs` - Subreddit syntax parsing

---

*This guide is intended for AI assistants working on this codebase. Last updated: 2025-12-30*
