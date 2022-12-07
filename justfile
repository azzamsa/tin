#!/usr/bin/env -S just --justfile

shebang := if os() == 'windows' { 'powershell.exe' } else { '/usr/bin/sh' }

set dotenv-load := true

alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the repository
setup:
    git cliff --version || cargo install git-cliff
    cargo nextest --version || cargo install --locked cargo-nextest
    cargo-set-version --help || cargo install cargo-edit
    sqlx --version || cargo install sqlx-cli --no-default-features --features postgres,native-tls
    cargo watch --version || cargo install cargo-watch
    cargo outdated --version || cargo install --locked cargo-outdated
    dprint --version || cargo install dprint

# Develop the app.
dev:
    cargo watch -x 'clippy --locked --all-targets --all-features'

# Develop the app.
run:
    cargo run

# Format the codebase.
fmt:
    cargo +nightly fmt --all
    dprint fmt --config .config/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    cargo +nightly fmt --all -- --check
    dprint check --config .config/dprint.json

# Lint the codebase.
lint:
    cargo clippy --locked --all-targets --all-features

# Check the documentation.
_doc-check:
    cargo doc --all-features --no-deps

# Run the unit tests.
_unit-test:
    cargo nextest run --lib

# Test the codebase.
test:
    cargo nextest run --all-targets --test-threads 1

_update-sqlx-schema:
    cargo sqlx prepare -- --lib

_check-sqlx-schema:
    cargo sqlx prepare --check -- --lib

# Setup the database schema.
_migrate-db:
    sqlx database create
    sqlx migrate run

# reset the database schema.
_reset-db:
    sqlx database drop && sqlx database create

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: fmt lint test _doc-check _update-sqlx-schema

# Check if the repository comply with the rules and ready to be pushed.
check: _check-sqlx-schema fmt-check lint test _doc-check

# Create a new release. Example `just release v2.2.0`
release version:
    bash scripts/release.sh {{ version }}

# Check dependencies health. Pass `--write` to uppgrade dependencies.
up arg="":
    #!{{ shebang }}
    if [ "{{ arg }}" = "--write" ]; then
    	cargo upgrade
    	cargo update
    else
        cargo outdated --root-deps-only
    fi;
