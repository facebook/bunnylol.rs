# `bunnylol.rs` — Smart browser bookmarks with Rust

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

Install from [crates.io](https://crates.io/crates/bunnylol):

```sh
# Install both CLI and server (3.9MB)
$ cargo install bunnylol

# Install just the CLI (1.4MB - recommended for terminal use only)
$ cargo install bunnylol --features cli --no-default-features

# Install just the server (3.6MB - recommended for web server deployments)
$ cargo install bunnylol --features server --no-default-features
```

Or build from source:

```sh
# Clone the repository
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs

# Install both CLI and server
$ cargo install --path .

# Install just the CLI
$ cargo install --path . --features cli --no-default-features

# Install just the server
$ cargo install --path . --features server --no-default-features
```

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
| `bunnylol tw @facebook` | Open Twitter profile |
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

Bunnylol uses different config file locations depending on how it's run:

**For CLI and manual server usage (`bunnylol serve`):**
- **Linux/macOS**: `~/.config/bunnylol/config.toml` (or `$XDG_CONFIG_HOME/bunnylol/config.toml` if set)
- **Windows**: `%APPDATA%\bunnylol\config.toml`

**For system service (`sudo bunnylol service install`):**
- **Linux**: `/etc/bunnylol/config.toml`

The config file is automatically created with sensible defaults when you first run bunnylol.

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

# Server configuration (for bunnylol serve) (optional)
[server]
port = 8000
address = "127.0.0.1"  # Use "0.0.0.0" for network access
log_level = "normal"   # Options: "normal", "debug", "critical", "off"
server_display_url = "https://bunny.example.com"  # Public URL shown on bindings page
```

### Platform-Specific Directory Structure

The CLI uses platform-appropriate directories for configuration and data:

| Platform | Type | Path |
|----------|------|------|
| **Linux/macOS** | User Config | `~/.config/bunnylol/config.toml`<br>(or `$XDG_CONFIG_HOME/bunnylol/config.toml`) |
| **Linux** | System Config | `/etc/bunnylol/config.toml`<br>(when running as system service) |
| **Linux/macOS** | Data | `~/.local/share/bunnylol/`<br>(or `$XDG_DATA_HOME/bunnylol/`) |
| **Windows** | Config | `%APPDATA%\bunnylol\config.toml` |
| **Windows** | Data | `%APPDATA%\bunnylol\` |

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
# Default: localhost only (127.0.0.1)
$ sudo bunnylol service install

# For network access (production servers)
$ sudo bunnylol service install --network

# The installer will:
# - Create /etc/systemd/system/bunnylol.service
# - Create /etc/bunnylol/config.toml with server settings
# - Enable autostart on boot
# - Start the service immediately

# Manage the service
$ sudo bunnylol service status
$ sudo bunnylol service logs -f
$ sudo bunnylol service restart

# Uninstall
$ sudo bunnylol service uninstall
```

**Network Access:**
- **Without `--network`** (default): Binds to `127.0.0.1` (localhost only, secure default)
- **With `--network`**: Binds to `0.0.0.0` (accessible from network, for production servers)

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
## Command Reference

<details>
<summary><strong>📚 Click to view all available commands (46 commands, 82+ bindings)</strong></summary>

<br>

### Development & Package Managers

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `gh` | — | Navigate to GitHub repositories | `gh facebook/react` |
| `gitlab` | `gl` | Navigate to GitLab projects or search GitLab | `gitlab gitlab-org/gitlab` |
| `cargo` | `crates` | Navigate to crates.io or search for Rust crates | `cargo serde` |
| `npm` | `npmjs` | Navigate to npmjs.com or search for npm packages | `npm react` |
| `pypi` | `pip` | Navigate to pypi.org or search for Python packages | `pypi requests` |
| `rubygems` | `gem`, `gems` | Navigate to rubygems.org or search for Ruby gems | `gem rails` |
| `go` | `golang`, `gopkg` | Navigate to pkg.go.dev or search for Go packages | `go http` |
| `nuget` | — | Navigate to nuget.org or search for .NET packages | `nuget newtonsoft` |
| `packagist` | `composer` | Navigate to packagist.org or search for PHP packages | `packagist symfony` |
| `brew` | `homebrew` | Navigate to formulae.brew.sh or search for Homebrew packages | `brew wget` |
| `choco` | `chocolatey` | Navigate to community.chocolatey.org or search for Windows packages | `choco git` |
| `dockerhub` | `docker` | Navigate to Docker Hub or search for container images | `docker nginx` |

### Programming Documentation

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `rust` | — | Navigate to Rust documentation or search Rust std docs | `rust HashMap` |
| `python` | `pydocs`, `py` | Navigate to Python documentation or search for Python resources | `python list` |
| `node` | `nodejs` | Navigate to Node.js API documentation or specific module docs | `node fs` |
| `godocs` | — | Navigate to Go language documentation | `godocs` |
| `hack` | — | Navigate to Hack documentation or search Hack docs | `hack async` |
| `mdn` | — | Navigate to MDN Web Docs or search for web development resources | `mdn flexbox` |
| `stackoverflow` | `so` | Navigate to Stack Overflow or search for programming questions | `so rust ownership` |

### Social Media

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `ig` | `instagram` | Navigate to Instagram profiles, search, or access Reels/Messages | `ig @instagram` |
| `tw` | — | Navigate to Twitter profiles or search Twitter | `tw @MetaOpenSource` |
| `threads` | — | Navigate to Threads profiles or search Threads | `threads @zuck` |
| `fb` | — | Navigate to Facebook pages or search Facebook | `fb Meta` |
| `li` | `linkedin` | Navigate to LinkedIn or search | `li software engineer` |
| `reddit` | `r` | Navigate to Reddit or search subreddits | `r r/rust` |
| `yt` | `youtube` | Navigate to YouTube or search for videos (supports: `studio`, `subs`) | `yt rust programming` |
| `wa` | `whatsapp` | Navigate to WhatsApp | `wa` |

### Google Services

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `g` | (default) | Search Google (default fallback for any unrecognized command) | `g rust programming` |
| `gmail` | `mail` | Navigate to Gmail | `mail` |
| `docs` | `gdoc` | Navigate to Google Docs | `docs` |
| `gsheets` | — | Navigate to Google Sheets | `gsheets` |
| `gslides` | — | Navigate to Google Slides | `gslides` |
| `gchat` | — | Navigate to Google Chat | `gchat` |
| `gmaps` | `maps` | Navigate to Google Maps or search for a location | `gmaps san francisco` |

### Meta / AI Services

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `meta` | `metaai` | Navigate to Meta, Meta AI, Meta Accounts Center, or Meta Pay | `meta accounts` |
| `claude` | — | Navigate to Claude AI (supports: `billing`, `cost`, `artifacts`, `chats`, `projects`) | `claude projects` |
| `chatgpt` | — | Navigate to ChatGPT | `chatgpt` |

### Shopping & Finance

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `az` | `amzn`, `azn`, `amazon` | Navigate to Amazon or search for products | `az headphones` |
| `rei` | — | Navigate to REI or search for outdoor gear | `rei hiking boots` |
| `schwab` | — | Charles Schwab shortcuts (`billpay`, `orders`, `trade`, `transfer`, `security`, `contact`) | `schwab trade` |
| `stock` | `stocks`, `finance`, `$<ticker>` | Look up stock prices on Yahoo Finance | `stock META` or `$META` |

### Other Services

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `1password` | `1p`, `onepassword` | 1Password home page | `1p` |
| `soundcloud` | `sc` | Navigate to SoundCloud (supports: `likes`) | `sc edm` |
| `wiki` | `wikipedia` | Search on Wikipedia | `wiki rust programming` |
| `ddg` | `duckduckgo` | Search DuckDuckGo | `ddg rust programming` |

### Bunnylol Development Tools

| Command | Aliases | Description | Example |
|---------|---------|-------------|---------|
| `bindings` | `commmands`, `list`, `bunny`, `cmd`, `cmds`, `help` | View all Bunnylol command bindings in a web portal | `bindings` |

### Special Syntax

- **Stock tickers**: Prefix with `$` → `$AAPL` redirects to Yahoo Finance
- **Twitter profiles**: Prefix with `@` → `tw @username`
- **Instagram profiles**: Prefix with `@` → `ig @username`
- **Threads profiles**: Prefix with `@` → `threads @username`
- **Subreddits**: Use `r/` prefix → `r r/rust`
- **Default fallback**: Any unrecognized command searches Google

</details>

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
