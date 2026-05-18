#!/usr/bin/env bash
#
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/release.sh [--bump patch|minor|major] [--dry-run]

Bumps the crate version, runs release checks, commits the version bump, and
creates an annotated v<version> tag and GitHub release. If --bump is not
provided, the script prompts for patch, minor, or major.

Options:
  --bump      Release increment to apply.
  --dry-run   Print the release steps without changing files or pushing.
USAGE
}

die() {
  echo "error: $*" >&2
  exit 1
}

bump=""
dry_run=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --bump)
      bump="${2:-}"
      [[ -n "$bump" ]] || die "--bump requires patch, minor, or major"
      shift
      ;;
    --dry-run)
      dry_run=1
      ;;
    *)
      die "unknown argument: $1"
      ;;
  esac
  shift
done

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

ensure_main_is_pushed() {
  local branch upstream local_sha upstream_sha merge_base

  branch="$(git branch --show-current)"
  if [[ "$branch" != "main" ]]; then
    die "release must be run from main; current branch is ${branch:-detached HEAD}"
  fi

  upstream="$(git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' 2>/dev/null)" ||
    die "main must have an upstream branch configured"

  if [[ "$dry_run" -eq 1 ]]; then
    echo "DRY RUN: would run git fetch --tags --prune before comparing main to ${upstream}"
  else
    git fetch --tags --prune
  fi

  local_sha="$(git rev-parse HEAD)"
  upstream_sha="$(git rev-parse "$upstream")"
  merge_base="$(git merge-base HEAD "$upstream")"

  if [[ "$local_sha" == "$upstream_sha" ]]; then
    return
  fi

  if [[ "$merge_base" == "$upstream_sha" ]]; then
    die "main has unpushed commits; push them before releasing"
  fi

  if [[ "$merge_base" == "$local_sha" ]]; then
    die "main is behind ${upstream}; pull or rebase before releasing"
  fi

  die "main has diverged from ${upstream}; resolve it before releasing"
}

require_release_tools() {
  command -v jq >/dev/null ||
    die "jq is required to read cargo metadata"

  command -v curl >/dev/null ||
    die "curl is required to verify crates.io authentication"

  command -v python3 >/dev/null ||
    die "python3 is required to parse Cargo credentials TOML"

  command -v gh >/dev/null ||
    die "gh is required to create the GitHub release"

  gh auth status >/dev/null 2>&1 ||
    die "gh must be authenticated to create the GitHub release; run: gh auth login"
}

ensure_cargo_set_version() {
  if cargo set-version --help >/dev/null 2>&1; then
    return
  fi

  echo "cargo set-version is missing; installing cargo-edit..."
  cargo install cargo-edit --locked

  if ! cargo set-version --help >/dev/null 2>&1; then
    die "cargo-edit installed, but cargo set-version is still unavailable"
  fi
}

credential_file() {
  local cargo_home
  cargo_home="${CARGO_HOME:-$HOME/.cargo}"

  if [[ -f "$cargo_home/credentials.toml" ]]; then
    echo "$cargo_home/credentials.toml"
    return
  fi

  if [[ -f "$cargo_home/credentials" ]]; then
    echo "$cargo_home/credentials"
    return
  fi
}

cargo_registry_token() {
  local file

  if [[ -n "${CARGO_REGISTRY_TOKEN:-}" ]]; then
    printf "%s" "$CARGO_REGISTRY_TOKEN"
    return
  fi

  if [[ -n "${CARGO_REGISTRIES_CRATES_IO_TOKEN:-}" ]]; then
    printf "%s" "$CARGO_REGISTRIES_CRATES_IO_TOKEN"
    return
  fi

  file="$(credential_file)"
  if [[ -z "$file" ]]; then
    return 1
  fi

  python3 - "$file" <<'PY'
import sys

try:
    import tomllib
except ModuleNotFoundError:
    sys.exit(2)

with open(sys.argv[1], "rb") as f:
    data = tomllib.load(f)

token = (
    data.get("registry", {}).get("token")
    or data.get("registries", {}).get("crates-io", {}).get("token")
    or data.get("token")
)

if token:
    print(token, end="")
PY
}

require_crates_io_auth() {
  local http_status token

  token="$(cargo_registry_token || true)"
  if [[ -z "$token" ]]; then
    die "crates.io token not found; run cargo login or set CARGO_REGISTRY_TOKEN"
  fi

  http_status="$(
    curl \
      --silent \
      --show-error \
      --output /dev/null \
      --write-out "%{http_code}" \
      --header "Authorization: $token" \
      --header "User-Agent: bunnylol-release-script" \
      https://crates.io/api/v1/me
  )" || die "failed to verify crates.io authentication"

  if [[ "$http_status" != "200" ]]; then
    die "crates.io authentication check failed with HTTP ${http_status}; run cargo login with a valid publish token"
  fi
}

require_metadata_tools() {
  command -v jq >/dev/null ||
    die "jq is required to read cargo metadata"
}

current_package_version() {
  cargo metadata --no-deps --format-version 1 |
    jq -r '.packages[] | select(.source == null) | .version' |
    head -1
}

release_tag_prefix() {
  cargo metadata --no-deps --format-version 1 |
    jq -r '.packages[] | select(.source == null) | .metadata.release["tag-prefix"] // "v"' |
    head -1
}

next_version_for_bump() {
  local current kind
  current="$1"
  kind="$2"

  if [[ ! "$current" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
    die "current version must be plain semver MAJOR.MINOR.PATCH; got ${current}"
  fi

  local major="${BASH_REMATCH[1]}"
  local minor="${BASH_REMATCH[2]}"
  local patch="${BASH_REMATCH[3]}"

  case "$kind" in
    major)
      echo "$((major + 1)).0.0"
      ;;
    minor)
      echo "${major}.$((minor + 1)).0"
      ;;
    patch)
      echo "${major}.${minor}.$((patch + 1))"
      ;;
    *)
      die "bump must be patch, minor, or major"
      ;;
  esac
}

release_tag_for_version() {
  local version prefix tag
  version="$1"
  prefix="$2"
  tag="${prefix}${version}"

  if [[ ! "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    die "release tag must be v-prefixed semver; got ${tag}"
  fi

  echo "$tag"
}

prompt_for_bump() {
  local current patch_version minor_version major_version choice
  current="$1"
  patch_version="$(next_version_for_bump "$current" patch)"
  minor_version="$(next_version_for_bump "$current" minor)"
  major_version="$(next_version_for_bump "$current" major)"

  echo "Current version: ${current}" >&2
  echo "Select release type:" >&2
  echo "  1) patch -> ${patch_version}" >&2
  echo "  2) minor -> ${minor_version}" >&2
  echo "  3) major -> ${major_version}" >&2

  while true; do
    read -r -p "Release type [patch]: " choice
    choice="${choice:-patch}"
    case "$choice" in
      1|p|patch)
        echo "patch"
        return
        ;;
      2|m|minor)
        echo "minor"
        return
        ;;
      3|M|major)
        echo "major"
        return
        ;;
      *)
        echo "Please enter patch, minor, or major." >&2
        ;;
    esac
  done
}

print_release_summary() {
  local current version tag mode publish_label
  current="$1"
  version="$2"
  tag="$3"

  if [[ "$dry_run" -eq 1 ]]; then
    mode="dry run"
    publish_label="no; dry runs never publish"
  else
    mode="release"
    publish_label="yes"
  fi

  cat <<SUMMARY
Release plan
  Mode: ${mode}
  Version: ${current} -> ${version}
  Tag: ${tag}
  Publish to crates.io: ${publish_label}

The script will verify that:
  - The working tree is clean.
  - The current branch is main.
  - main is in sync with its upstream branch.
  - jq, curl, python3, and gh are available.
  - gh is authenticated.
  - crates.io authentication is valid.
  - ${tag} does not already exist.

For a real release, the script will:
  - Install cargo-edit if cargo set-version is missing.
  - Run cargo set-version ${version}.
  - Run cargo metadata, git diff --check, cargo fmt, cargo clippy, cargo test, and cargo package.
  - Commit Cargo.toml and Cargo.lock.
  - Create annotated tag ${tag}.
  - Push main and ${tag}.
  - Publish ${version} to crates.io.
  - Create a GitHub release for ${tag} with generated release notes.

Dry run mode only prints the steps; it does not change files, run checks, commit, tag, push, publish, or create a GitHub release.
SUMMARY
}

confirm_release() {
  local answer
  read -r -p "Continue? [y/N] " answer
  case "$answer" in
    y|Y|yes|YES)
      ;;
    *)
      die "release cancelled"
      ;;
  esac
}

require_metadata_tools

current_version="$(current_package_version)"
tag_prefix="$(release_tag_prefix)"

if [[ -z "$current_version" ]]; then
  die "could not read current crate version"
fi

if [[ "$tag_prefix" != "v" ]]; then
  die "Cargo.toml package.metadata.release.tag-prefix must be \"v\"; got ${tag_prefix}"
fi

if [[ -z "$bump" ]]; then
  bump="$(prompt_for_bump "$current_version")"
fi

version="$(next_version_for_bump "$current_version" "$bump")"
tag="$(release_tag_for_version "$version" "$tag_prefix")"

if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
  die "tag already exists: ${tag}"
fi

print_release_summary "$current_version" "$version" "$tag"
confirm_release

if [[ "$dry_run" -eq 1 ]]; then
  echo "DRY RUN: would verify the working tree is clean"
  echo "DRY RUN: would verify the current branch is main"
  echo "DRY RUN: would verify main is in sync with its upstream branch"
  echo "DRY RUN: would verify jq, curl, python3, and gh are available"
  echo "DRY RUN: would verify gh is authenticated"
  echo "DRY RUN: would verify crates.io authentication"
  echo "DRY RUN: would verify ${tag} does not already exist"
  echo "DRY RUN: release ${current_version} -> ${version} (${tag})"
  echo "DRY RUN: would install cargo-edit if cargo set-version is missing"
  echo "DRY RUN: would run cargo set-version ${version}"
  echo "DRY RUN: would run cargo metadata --no-deps --format-version 1"
  echo "DRY RUN: would verify Cargo.toml/Cargo.lock changed and package version is ${version}"
  echo "DRY RUN: would run git diff --check"
  echo "DRY RUN: would run cargo fmt --all -- --check"
  echo "DRY RUN: would run cargo clippy --all-features -- -D warnings"
  echo "DRY RUN: would run cargo test --all-features"
  echo "DRY RUN: would run cargo package --allow-dirty"
  echo "DRY RUN: would commit Cargo.toml and Cargo.lock with message: release: ${tag}"
  echo "DRY RUN: would create annotated tag ${tag}"
  echo "DRY RUN: would run git push origin main ${tag}"
  echo "DRY RUN: would run cargo publish"
  echo "DRY RUN: would run gh release create ${tag} --verify-tag --title 'Release ${tag}' --generate-notes"
  echo "Dry run complete; no files were changed."
  exit 0
fi

if [[ -n "$(git status --porcelain)" ]]; then
  git status --short >&2
  die "working tree must be clean before releasing"
fi

ensure_main_is_pushed
require_release_tools
require_crates_io_auth
ensure_cargo_set_version

if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
  die "tag already exists after fetching tags: ${tag}"
fi

cargo set-version "$version"
cargo metadata --no-deps --format-version 1 >/dev/null

updated_version="$(current_package_version)"
if [[ "$updated_version" != "$version" ]]; then
  die "version bump failed: expected ${version}, found ${updated_version}"
fi

if git diff --quiet -- Cargo.toml Cargo.lock; then
  die "version bump did not change Cargo.toml or Cargo.lock"
fi

git diff --check
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
cargo package --allow-dirty

git add Cargo.toml Cargo.lock
git commit -m "release: ${tag}" \
  -m "Bump bunnylol to ${version} and verify the crate package."
git tag -a "$tag" -m "Release ${tag}"

git push origin main "$tag"
cargo publish
gh release create "$tag" --verify-tag --title "Release ${tag}" --generate-notes

echo "Created, pushed, and published GitHub release ${tag}."
