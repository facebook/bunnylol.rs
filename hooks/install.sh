#!/bin/bash
# Install git hooks for bunnylol.rs

set -e

HOOKS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

echo "Installing git hooks..."
echo ""

# Install pre-commit hook
if [ -f "$GIT_HOOKS_DIR/pre-commit" ]; then
    echo "⚠️  Warning: pre-commit hook already exists"
    echo "   Existing: $GIT_HOOKS_DIR/pre-commit"
    echo "   Overwrite? [y/N]"
    read -r response
    if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "Skipping pre-commit hook installation"
        exit 0
    fi
fi

cp "$HOOKS_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"

echo "✅ Installed pre-commit hook"
echo ""
echo "The hook will run before each commit:"
echo "  - cargo check (compilation check)"
echo "  - cargo fmt --check (formatting check)"
echo "  - cargo clippy (linter)"
echo "  - cargo test (full test suite)"
echo ""
echo "To bypass the hook (not recommended), use: git commit --no-verify"
