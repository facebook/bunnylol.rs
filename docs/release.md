<!--
Copyright (c) Meta Platforms, Inc. and affiliates.

This source code is licensed under the MIT license found in the
LICENSE file in the root directory of this source tree.
-->

# Release Process

Use `scripts/release.sh` for every release. The script owns the version bump,
release checks, `v`-prefixed tag, crates.io publish, push, and GitHub release.

Run a plan-only dry run first:

```bash
scripts/release.sh --bump patch --dry-run
```

Cut the release:

```bash
scripts/release.sh --bump patch
```

Use `--bump minor` or `--bump major` when the release warrants it. If `--bump`
is omitted, the script prompts for patch, minor, or major.

The real release flow:

1. Confirms the working tree is clean.
2. Confirms the current branch is `main`.
3. Confirms `main` is in sync with its upstream branch.
4. Installs `cargo-edit` if `cargo set-version` is missing.
5. Bumps `Cargo.toml` and `Cargo.lock`.
6. Runs `cargo metadata`, `git diff --check`, `cargo fmt`, `cargo clippy`,
   `cargo test`, and `cargo package`.
7. Creates a `chore: release vX.Y.Z` commit.
8. Creates an annotated `vX.Y.Z` tag.
9. Pushes `main` and the tag.
10. Publishes to crates.io.
11. Creates the GitHub release with generated release notes.
