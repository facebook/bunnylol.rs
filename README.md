# `bunnylol.rs` -- Smart browser bookmarks with Rust

[![Crates.io](https://img.shields.io/crates/v/bunnylol.svg?style=flat-square)](https://crates.io/crates/bunnylol)
[![Downloads](https://img.shields.io/crates/d/bunnylol.svg?style=flat-square)](https://crates.io/crates/bunnylol)
[![Contributors](https://img.shields.io/github/contributors/facebook/bunnylol.rs.svg?style=flat-square)](https://github.com/facebook/bunnylol.rs/graphs/contributors)
[![Stargazers](https://img.shields.io/github/stars/facebook/bunnylol.rs.svg?style=flat-square)](https://github.com/facebook/bunnylol.rs/stargazers)
[![License](https://img.shields.io/github/license/facebook/bunnylol.rs?style=flat-square)](https://github.com/facebook/bunnylol.rs/blob/master/LICENSE)

<p align="center">
    A modern rust clone of <a href="https://github.com/ccheever/bunny1">bunny1</a>.
</p>

## Demo

Enter `gh facebook/react` in your browser's address bar to open the React repository on GitHub.

![bunnylol.rs demo](demo.gif)

Or run the CLI:

```sh
$ bunnylol gh facebook/react
```

## Installation

Clone and build from source:

```sh
# Clone the repository
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs

# Install both CLI and server (3.9MB)
$ cargo install --path .

# Install just the CLI (1.4MB - recommended for terminal use only)
$ cargo install --path . --features cli --no-default-features

# Install just the server (3.6MB - recommended for web server deployments)
$ cargo install --path . --features server --no-default-features
```

<!-- `bunnylol` will be available on [crates.io](https://crates.io/crates/bunnylol): -->
<!---->
<!-- ```sh -->
<!-- # Install both CLI and server (3.9MB) -->
<!-- $ cargo install bunnylol -->
<!---->
<!-- # Install just the CLI (1.4MB - recommended for terminal use only) -->
<!-- $ cargo install bunnylol --features cli --no-default-features -->
<!---->
<!-- # Install just the server (3.6MB - recommended for web server deployments) -->
<!-- $ cargo install bunnylol --features server --no-default-features -->
<!-- ``` -->

**Which one should I use?**
- **CLI only**: If you only want terminal shortcuts and don't need the web server (1.4MB, faster to compile)
- **Server only**: For deploying on a server where you don't need CLI features (3.6MB)
- **Both** (default): Get everything - use the CLI locally and run the web server (3.9MB)

<!-- TABLE OF CONTENTS -->
## Table of Contents

  - [Demo](#demo)
  - [Installation](#installation)
  - [CLI Quickstart](#cli-quickstart)
  - [CLI Configuration](#cli-configuration)
  - [Web Server Quickstart](#quickstart---web-server)
    - [Installing as a System Service](#installing-as-a-system-service)
  - [Setting bunnylol as Default Search Engine](#setting-bunnylol-to-be-your-default-search-engine)
  - [Command Examples](#other-command-examples)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Manual Setup](#manual-setup)
  - [Deployment](#deployment-with-docker)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgments](#acknowledgements)



## CLI Quickstart

Use `bunnylol` to open URLs directly from your terminal!

### Basic Usage

```sh
# Open GitHub
$ bunnylol gh

# Open Instagram Reels
$ bunnylol ig reels

# Open a specific GitHub repository
$ bunnylol gh facebook/react

# Preview URL without opening browser (dry-run)
$ bunnylol --dry-run gh facebook/react
# Output: https://github.com/facebook/react

# List all available commands with a beautiful table
$ bunnylol list
```

### Quick Examples

| CLI Command | What it does |
|-------------|-------------|
| `bunnylol gh` | Open GitHub homepage |
| `bunnylol gh facebook/react` | Open facebook/react repository |
| `bunnylol ig reels` | Open Instagram Reels |
| `bunnylol tw @elonmusk` | Open Twitter profile |
| `bunnylol r r/rust` | Open r/rust subreddit |
| `bunnylol --dry-run meta ai` | Print Meta AI URL without opening |
| `bunnylol --help` | Show help information |
| `bunnylol --version` | Show version information |
| `bunnylol list` | Display all commands in a formatted table |

### Recommended: Create a Shell Alias

For even faster access, add an alias to your shell configuration:

```sh
# Add to ~/.bashrc or ~/.zshrc
alias b="bunnylol"

# Then use it like this:
$ b ig reels
$ b gh facebook/react
$ b list
```

## CLI Configuration

The bunnylol CLI supports optional configuration via a TOML file following the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).

### Configuration File Location

The config file is located at:
- **Linux/macOS**: `~/.config/bunnylol/config.toml` (or `$XDG_CONFIG_HOME/bunnylol/config.toml` if set)
- **Windows**: `%APPDATA%\bunnylol\config.toml`

### Configuration Features

The CLI works perfectly fine without any configuration file. However, you can customize the following features:

#### 1. **Default Browser Selection**

Specify which browser to open URLs in:

```toml
# ~/.config/bunnylol/config.toml
browser = "firefox"  # or "chrome", "chromium", "safari", etc.
```

If not specified, the system default browser is used.

#### 2. **Custom Command Aliases**

Create your own personalized shortcuts:

```toml
[aliases]
work = "gh mycompany"
blog = "gh username/blog"
dotfiles = "gh username/dotfiles"
```

Then use them like any built-in command:
```sh
$ bunnylol work
# Opens: https://github.com/mycompany

$ bunnylol blog
# Opens: https://github.com/username/blog
```

#### 3. **Custom Default Search Engine**

Override Google as the fallback search engine:

```toml
default_search = "ddg"  # Options: "google" (default), "ddg", "bing"
```

When a command isn't recognized, it will search using your configured engine instead of Google.

#### 4. **Command History Tracking**

Track your recently used commands (enabled by default):

```toml
[history]
enabled = true
max_entries = 1000
```

History is stored at:
- **Linux/macOS**: `~/.local/share/bunnylol/history` (or `$XDG_DATA_HOME/bunnylol/history` if set)
- **Windows**: `%APPDATA%\bunnylol\history`

### Complete Configuration Example

Here's a full example with all available options:

```toml
# ~/.config/bunnylol/config.toml

# Browser to open URLs in (optional)
browser = "firefox"

# Custom command aliases (optional)
[aliases]
work = "gh mycompany"
blog = "gh username/blog"
dotfiles = "gh username/dotfiles"
notes = "gh username/notes"

# Default search engine when command not recognized (optional)
# Options: "google" (default), "ddg", "bing"
default_search = "ddg"

# Command history settings (optional)
[history]
enabled = true
max_entries = 1000
```

### Platform-Specific Directory Structure

The CLI uses platform-appropriate directories for configuration and data:

| Platform | Type | Path |
|----------|------|------|
| **Linux/macOS** | Config | `~/.config/bunnylol/config.toml`<br>(or `$XDG_CONFIG_HOME/bunnylol/config.toml`) |
| **Linux/macOS** | Data | `~/.local/share/bunnylol/`<br>(or `$XDG_DATA_HOME/bunnylol/`) |
| **Windows** | Config | `%APPDATA%\bunnylol\config.toml` |
| **Windows** | Data | `%APPDATA%\bunnylol\` |

### Creating Your First Config

To get started with a config file:

```sh
# Create the config directory
mkdir -p ~/.config/bunnylol

# Create a basic config file
cat > ~/.config/bunnylol/config.toml << 'EOF'
# Set your preferred browser
browser = "firefox"

# Add custom aliases
[aliases]
work = "gh yourcompany"

# Use DuckDuckGo for fallback searches
default_search = "ddg"
EOF

# Test it out!
bunnylol work
```

## Quickstart - Web Server

After [installing](#installation) bunnylol, start the server:

```sh
$ bunnylol serve
```

Or use Docker:

```sh
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs
$ docker compose up -d
```

Or build from source:

```sh
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs
$ cargo run -- serve
```

### Installing as a System Service

For production use on **Linux**, install bunnylol as a `systemd` service that starts automatically on boot:

```sh
# Install bunnylol first
$ cargo install bunnylol

# Install as system service (requires sudo, Linux only)
$ sudo bunnylol service install

# The installer will:
# - Create /etc/systemd/system/bunnylol.service
# - Enable autostart on boot
# - Start the service immediately

# Manage the service
$ sudo bunnylol service status
$ sudo bunnylol service logs -f
$ sudo bunnylol service restart

# Uninstall
$ sudo bunnylol service uninstall
```

The service installer works on:
- **Linux**: `systemd` (Ubuntu 16.04+, Debian 8+, CentOS 7+, etc.)

**macOS and Windows:** Use Docker instead (see above) or run `bunnylol serve` directly.

For more details, see the [Deployment Guide](deploy/DEPLOYMENT.md).

Open your web browser and navigate to `http://localhost:8000/?cmd=fb` to get redirected to Facebook.

Open `http://localhost:8000/?cmd=gh facebook/bunnylol.rs` to be redirected to this repo.

## Setting `bunnylol` to be your default search engine

You can set your default search engine to `http://localhost:8000/?cmd=%s` and use `bunnylol.rs` for everything. For this to work, you will need to have the server deployed and running locally or on a server.

**Note:** For best results, deploy bunnylol on a networked server accessible from all your devices, rather than just running it locally.

### Desktop Browsers

- [Guide for doing this in Desktop Chrome](https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop)
- [Guide for doing this in Desktop Firefox](https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox)

### Mobile Browsers

**Note:** iOS Safari does not support custom search engines, so you'll need to use Firefox (or another browser that does) instead.

#### iOS (Firefox)
1. Install Firefox and [set it as the default browser](https://support.covenanteyes.com/hc/en-us/articles/12223357002267-How-do-I-set-a-default-browser-on-an-iPhone)
2. Change your [default search engine in Firefox for iOS](https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios)

#### Android (Firefox)
- [Guide for managing default search engines in Firefox for Android](https://support.mozilla.org/en-US/kb/manage-my-default-search-engines-firefox-android)

<!-- USAGE EXAMPLES -->
## Other Command Examples

| Command | Usage Example | Description |
|---------|--------------|-------------|
| `bindings`, `commmands`, `list` | `bindings` | View all Bunnylol command bindings in a web portal |
| `gh` | `gh` | Redirects to github.com |
| `gh` | `gh username` | Redirects to github.com/username |
| `gh` | `gh username/repo` | Redirects to github.com/username/repo |
| `tw` | `tw` | Redirects to twitter.com |
| `tw` | `tw @username` | Redirects to twitter.com/username |
| `tw` | `tw search terms` | Searches Twitter for "search terms" |
| `r`, `reddit` | `r` | Redirects to reddit.com |
| `r`, `reddit` | `r search terms` | Searches Reddit for "search terms" |
| `r`, `reddit` | `r r/subreddit` | Redirects to reddit.com/r/subreddit |
| `r`, `reddit` | `r r/subreddit search terms` | Searches within a subreddit for "search terms" |
| `mail`, `gmail` | `mail` | Redirects to mail.google.com |
| `rei` | `rei` | Redirects to www.rei.com |
| `rei` | `rei search terms` | Searches REI for "search terms" |
| `fb` | `fb` | Redirects to facebook.com |
| `fb` | `fb page` | Redirects to facebook.com/page |
| `fb` | `fb search terms` | Searches Facebook for "search terms" |
| `ig`, `instagram` | `ig` | Redirects to instagram.com |
| `ig`, `instagram` | `ig @username` | Redirects to instagram.com/username |
| `ig`, `instagram` | `ig search terms` | Searches Instagram for "search terms" |
| `threads` | `threads` | Redirects to threads.net |
| `threads` | `threads @username` | Redirects to threads.net/@username |
| `threads` | `threads search terms` | Searches Threads for "search terms" |
| `wa`, `whatsapp` | `wa` | Redirects to whatsapp.com |
| `meta`, `metaai` | `meta` | Redirects to meta.com |
| `meta`, `metaai` | `meta accounts` | Redirects to Meta Accounts Center |
| `meta`, `metaai` | `meta ai` or `metaai` | Redirects to meta.ai |
| `cargo`, `crates` | `cargo` | Redirects to crates.io |
| `cargo`, `crates` | `cargo serde` | Searches crates.io for "serde" |
| `cargo`, `crates` | `cargo settings` | Redirects to crates.io/settings/profile |
| `cargo`, `crates` | `cargo tokens` | Redirects to crates.io/settings/tokens |
| `npm`, `npmjs` | `npm` | Redirects to npmjs.com |
| `npm`, `npmjs` | `npm react` | Searches npmjs.com for "react" |
| `claude` | `claude` | Redirects to claude.ai |
| `chatgpt` | `chatgpt` | Redirects to chatgpt.com |
| `rust` | `rust` | Redirects to Rust std documentation |
| `rust` | `rust HashMap` | Searches Rust std docs for "HashMap" |
| `hack` | `hack` | Redirects to Hack documentation |
| `hack` | `hack async` | Searches Hack docs for "async" |
| `az`, `amzn`, `azn`, `amazon` | `az` | Redirects to amazon.com |
| `az`, `amzn`, `azn`, `amazon` | `az headphones` | Searches Amazon for "headphones" |
| `yt`, `youtube` | `yt` | Redirects to youtube.com |
| `yt`, `youtube` | `yt search terms` | Searches YouTube for videos |
| `yt`, `youtube` | `yt studio` | Redirects to YouTube Studio |
| `yt`, `youtube` | `yt subscriptions` or `yt subs` | Redirects to YouTube subscriptions feed |
| `docs`, `gdoc` | `docs` | Redirects to Google Docs |
| `gsheets` | `gsheets` | Redirects to Google Sheets |
| `gslides` | `gslides` | Redirects to Google Slides |
| `gchat` | `gchat` | Redirects to Google Chat |
| `devbunny` | `devbunny command` | Redirects to localhost:8000/?cmd=command (for testing) |
| `g` | `g search terms` | Searches Google for "search terms" |
| (default) | `any search terms` | Searches Google for "any search terms" (default fallback) |

### Built With

* [Rust](https://www.rust-lang.org/)
* [Rocket](https://rocket.rs/) - Web framework
* [Leptos](https://leptos.dev/) - Frontend framework for the bindings page
* [clap](https://github.com/clap-rs/clap) - CLI argument parser
* [tabled](https://github.com/zhiburt/tabled) - Beautiful terminal tables

<!-- GETTING STARTED -->
## Getting Started

See the [Installation](#installation) section to install bunnylol from crates.io.

To build from source or contribute to the project, see [Manual Setup](#manual-setup) below.

### Manual Setup

Make sure you have [Rust installed](https://rust-lang.org/tools/install/).

```sh
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs

# Run the web server
$ cargo run -- serve

# OR run the CLI (in a separate terminal)
$ cargo run -- gh facebook/react

# OR install globally for easier access
$ cargo install --path .
```

## Deployment with Docker

`Bunnylol` is designed to be easy to deploy anywhere using Docker.

```sh
# run on default port 8000
$ docker compose up -d

# run on custom port 9000
$BUNNYLOL_PORT=9000·docker compose up
```

The application will be running at `http://localhost:8000` by default.

### Auto-start on Boot (Linux)

Docker containers can automatically start on system boot:

1. Enable Docker service: `sudo systemctl enable docker`
2. Use restart policy in `docker-compose.yml`:
   ```yaml
   services:
     bunnylol:
       restart: unless-stopped
   ```

### Where to Deploy

Docker makes it easy to deploy anywhere:
- Any cloud provider (AWS, GCP, Azure, DigitalOcean, Hetzner, etc.)
- VPS / home servers

For detailed deployment instructions, reverse proxy setup, and troubleshooting, see the **[Deployment Guide](deployment/DEPLOYMENT.md)**.

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

## Acknowledgments

* [The Rust Community](https://www.rust-lang.org/community)
* [Rocket.rs](https://rocket.rs/)
* [@othneildrew](https://github.com/othneildrew) - for the [README template](https://github.com/othneildrew/Best-README-Template)
