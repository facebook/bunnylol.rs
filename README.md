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
    A tool that lets you write smart bookmarks (in Rust) and share them across all of your browsers.
    <br />
    A modern rust clone of <a href="https://github.com/ccheever/bunny1">bunny1  </a>.
</p>

<!-- TABLE OF CONTENTS -->
## Table of Contents

  - [Demo](#demo)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Manual Setup](#manual-setup)
    - [Running](#running)
    - [Testing](#testing)
  - [Usage](#usage)
  - [Deployment](#deployment)
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

Open your web browser and go to `http://localhost:8000/?cmd=fb` get redirected to Facebook.

Open `http://localhost:8000/?cmd=gh facebook/bunnylol.rs` to be redirected to this repo.

## Setting `bunnylol` to be your default search engine

You can set your default search engine to `http://localhost:8000/?cmd=%s` and use `bunnylol.rs` for everything. For this to work, you will need to have the server deployed and running locally or on a server.

**Note:** For best results, deploy bunnylol on a networked server accessible from all your devices, rather than just running it locally.

### Desktop Browsers

- [Guide for doing this in Desktop Chrome](https://support.google.com/chrome/answer/95426?hl=en&co=GENIE.Platform%3DDesktop)
- [Guide for doing this in Desktop Firefox](https://support.mozilla.org/en-US/kb/add-custom-search-engine-firefox)

### Mobile Browsers

**Note:** iOS Safari does not support custom search engines. On iOS, you'll need to use Firefox instead.

#### iOS (Firefox)
1. Install Firefox and [set it as the default browser](https://support.covenanteyes.com/hc/en-us/articles/12223357002267-How-do-I-set-a-default-browser-on-an-iPhone)
2. Change your [default search engine in Firefox for iOS](https://support.mozilla.org/en-US/kb/change-your-default-search-engine-firefox-ios)

#### Android (Firefox)
- [Guide for managing default search engines in Firefox for Android](https://support.mozilla.org/en-US/kb/manage-my-default-search-engines-firefox-android)

<!-- USAGE EXAMPLES -->
## Other Command Examples

| Command | Usage Example | Description |
|---------|--------------|-------------|
| `bindings` | `bindings` | Shows a table of supported bindings / commands |
| `gh` | `gh` | Redirects to github.com |
| `gh` | `gh username` | Redirects to github.com/username |
| `gh` | `gh username/repo` | Redirects to github.com/username/repo |
| `tw` | `tw` | Redirects to twitter.com |
| `tw` | `tw @username` | Redirects to twitter.com/username |
| `tw` | `tw search terms` | Searches Twitter for "search terms" |
| `r` | `r` | Redirects to reddit.com |
| `r` | `r search terms` | Searches Reddit for "search terms" |
| `r` | `r r/subreddit` | Redirects to reddit.com/r/subreddit |
| `r` | `r r/subreddit search terms` | Searches within a subreddit for "search terms" |
| `mail` | `mail` | Redirects to mail.google.com |
| `rei` | `rei` | Redirects to www.rei.com |
| `rei` | `rei search terms` | Searches REI for "search terms" |
| `devbunny` | `devbunny command` | Redirects to localhost:8000/?cmd=command (for testing) |
| `g` | `any search terms` | Searches Google for "any search terms" |
| (default) | `any search terms` | Searches Google for "any search terms" |

### Built With

* [Rust](https://www.rust-lang.org/)
* [Rocket](https://rocket.rs/)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow the simple steps under either of the following sections:
- [Manual Setup](#manual-setup) – follow this if you prefer to install all dependencies locally.

### Manual Setup

Make sure you have [Rust installed](https://rust-lang.org/tools/install/).

```sh
$ git clone https://github.com/facebook/bunnylol.rs.git
$ cd bunnylol.rs
$ cargo run
```


## Deployment with Docker

`Bunnylol` is designed to be easy to deploy anywhere using Docker.

```sh
docker-compose up -d
```

The application will be running at `http://localhost:8000` by default.

### Auto-start on Boot (Linux)

Docker containers can automatically start on system boot:

1. Enable Docker service: `sudo systemctl enable docker` (or )
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


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/facebook/bunnylol.rs.svg?style=flat-square
[contributors-url]: https://github.com/facebook/bunnylol.rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/facebook/bunnylol.rs.svg?style=flat-square
[forks-url]: https://github.com/facebook/bunnylol.rs/network/members
[stars-shield]: https://img.shields.io/github/stars/facebook/bunnylol.rs.svg?style=flat-square
[stars-url]: https://github.com/facebook/bunnylol.rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/facebook/bunnylol.rs.svg?style=flat-square
[issues-url]: https://github.com/facebook/bunnylol.rs/issues
[license-shield]: https://img.shields.io/github/license/facebook/bunnylol.rs?style=flat-square
[license-url]: https://github.com/facebook/bunnylol.rs/blob/master/LICENSE
[product-screenshot]: demo.gif
