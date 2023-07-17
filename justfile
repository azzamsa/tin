#!/usr/bin/env -S just --justfile

set dotenv-load := true

alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup: _areyousure _setup-dev

# Setup the development tools.
_setup-dev:
    sudo apt install --yes  pkg-config libssl-dev

    just _cargo-install 'cargo-edit cargo-nextest cargo-outdated cargo-watch dprint git-cliff spacer'
    just _cargo-install 'sqlx-cli'

# Develop the app.
dev:
    cargo watch -x 'clippy --locked --all-targets --all-features' | spacer

# Build the docker image.
build-image:
    podman build -t tin:latest --build-arg VCS_REVISION=$(git rev-parse --short HEAD) .

# Develop the app.
run:
    cargo run

# Run the docker image.
run-image:
    podman-compose --file docker-compose.local.yml up

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt --config configs/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check --config configs/dprint.json

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
    cargo nextest run --config-file configs/nextest.toml

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
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade
        cargo update
    else
        cargo outdated --root-deps-only
    fi;

[windows]
up arg="":
    #!powershell.exe
    if ( "tool" -eq "--write") {
        cargo upgrade
        cargo update
    }
    else {
        cargo outdated --root-deps-only
    }

#
# Helper
#

[unix]
_cargo-install tool:
    #!/usr/bin/env bash
    if command -v cargo-binstall >/dev/null 2>&1; then
        echo "cargo-binstall..."
        cargo binstall --no-confirm --no-symlinks {{ tool }}
    else
        echo "Building from source"
        cargo install --locked {{ tool }}
    fi

[unix]
_areyousure:
    #!/usr/bin/env bash
    echo -e "This command will alter your system. ⚠️
    You are advised to run in inside containerized environment.
    Such as [toolbx](https://containertoolbx.org/).

    If you are unsure. Run the installation commands manually.
    Take a look at the 'setup' recipe in the Justfile.\n"

    read -p "Are you sure you want to proceed? (Y/n) " response;
    if [[ $response =~ ^[Yy] ]]; then
        echo "Continue!";
    else
        echo "Cancelled!";
        exit 1;
    fi
