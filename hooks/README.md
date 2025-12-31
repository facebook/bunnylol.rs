# Git Hooks

This directory contains git hooks for the bunnylol.rs project.

## Installation

To install the pre-commit hook that runs tests before each commit:

```bash
./hooks/install.sh
```

## What it does

The pre-commit hook will:
1. Run `cargo fmt --all` (automatically format code)
2. Re-stage any formatted Rust files
3. Run `cargo check --all-features` (fast compilation check)
4. Run `cargo clippy --all-features -- -D warnings` (linter)
5. Run `cargo test --all-features` (full test suite)

If any of these fail, the commit will be blocked.

**Note:** The hook automatically formats your code before committing, so you don't need to run `cargo fmt` manually.

## Bypassing the hook

If you need to commit without running tests (not recommended):

```bash
git commit --no-verify
```

## Uninstalling

To remove the hook:

```bash
rm .git/hooks/pre-commit
```
