# bunnylol.rs

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]

<br />
<p align="center">
  <h3 align="center">bunnylol.rs</h3>

  <p align="center">
    <br />
    A tool that lets you write smart bookmarks (in Rust) and share them across all of your browsers.
    <br />
    A modern rust clone of <a href="https://github.com/ccheever/bunny1">bunny1  </a>.
  </p>
</p>

<!-- TABLE OF CONTENTS -->
## Table of Contents

- [bunnylol.rs](#bunnylolrs)
  - [Table of Contents](#table-of-contents)
  - [About the Project](#about-the-project)
  - [Demo](#demo)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Manual Setup](#manual-setup)
    - [VSCode Dev Container Setup](#vscode-dev-container-setup)
    - [Running](#running)
    - [Testing](#testing)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)
  - [Acknowledgements](#acknowledgements)

## Demo

![bunnylol.rs demo][product-screenshot]

This is what `bunnylol.rs` looks like in action.

## Quickstart

```
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs
$ cargo run
```

Open your web browser and go to `http://localhost:8000/search/fb` get redirected to Facebook.

Open `http://localhost:8000/?cmd=gh facebook/bunnylol.rs` to get redirected to this repo.

You can set your default search engine to `http://localhost:8000/?cmd=%s` and use bunnylol.rs. [Here is a guide for doing this in Chrome](https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop). This can work with a local or remote server deployment.

### Built With

* [Rust](https://www.rust-lang.org/)
* [Rocket](https://rocket.rs/)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow the simple steps under either of the following sections:
- [Manual Setup](#manual-setup) – follow this if you prefer to install all dependencies locally.
- [VSCode Dev Container Setup](#vscode-dev-container-setup) – follow this to run the project in an isolated development environment inside a Docker container, pre-installed with all dependencies.

### Manual Setup

#### Prerequisites

Make sure you have [Rust installed](https://rust-lang.org/tools/install/).

#### Installation

1. Clone `bunnylol.rs`
```sh
git clone https://github.com/facebook/bunnylol.rs.git
```
2. Build the project
```sh
cargo build
```
4. Follow the instructions in the [Running](#running) section.

### VSCode Dev Container Setup

#### Prerequisites

This requires VSCode, Docker and the Remote Development extension pack. For more details see [the official docs](https://code.visualstudio.com/docs/remote/containers#_system-requirements).

#### Spinning Up The Environment

- Follow [the official guide](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-a-git-repository-or-github-pr-in-an-isolated-container-volume) to open this repository inside a dev container.

### Running

1. Run the project
```sh
cargo run
```
2. Visit [localhost:8000](http://localhost:8000/)
3. To test a command, go to [localhost:8000/search?cmd=tw](http://localhost:8000/search?cmd=tw) and you should be redirected to Twitter

### Testing

Run the following command
```sh
cargo test
```

<!-- USAGE EXAMPLES -->
## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by your command.

The following commands are supported by `bunnylol.rs`:
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username
- "gh" -> redirects to github.com
- "gh username" -> redirects to github.com/username
- "gh username/repo" -> redirects to github.com/username/repo

Everything else redirects to a google search with your query.

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

<!-- LICENSE -->
## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [The Rust Community](https://www.rust-lang.org/community)
* [Rocket.rs](https://rocket.rs/)
* [@othneildrew](https://github.com/othneildrew) - for the [README template](https://github.com/othneildrew/Best-README-Template)
