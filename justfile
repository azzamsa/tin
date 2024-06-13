#!/usr/bin/env -S just --justfile

set dotenv-load := true

alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test
alias c := comply
alias k := check

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup: _areyousure
    just _cargo-install 'cargo-edit cargo-nextest cargo-outdated dprint git-cliff bacon typos-cli'
    just _cargo-install 'sqlx-cli'

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: _doc-check _update-sqlx-schema fmt lint test

# Check if the repository comply with the rules and ready to be pushed.
check: _doc-check _check-sqlx-schema fmt-check lint test

# Develop the app.
dev:
    bacon

# Run the app.
run:
    cargo run

# Build the container image.
image-build:
    podman build --tag tin:latest --build-arg VCS_REVISION=$(git rev-parse --short HEAD) .

# Run the container.
image-start service="":
    podman-compose --file compose.local.yml up {{ service }} -d

# Stop the container.
image-stop:
    podman-compose --file compose.local.yml down

# Restart the containers.
image-restart:
    just image-stop
    just image-run

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check

# Lint the codebase.
lint:
    cargo clippy --all-targets --all-features
    typos --config configs/typos.toml

# Test the codebase.
test:
    cargo nextest run --config-file configs/nextest.toml

# Run the unit tests.
test-unit:
    cargo nextest run --lib

# Create a new release. Example `cargo-release release minor --tag-name v0.2.0`
release level:
    cargo-release release {{ level }} --execute

# Make sure the repo is ready for release
release-check level: check
    just up
    cargo-release release {{ level }}

# Setup the database schema.
db-migrate:
    sqlx database create
    # sqlx migrate run # auto migration enabled

# reset the database schema.
db-reset:
    sqlx database drop && sqlx database create

# Check the documentation.
_doc-check:
    cargo doc --all-features --no-deps

_update-sqlx-schema:
    cargo sqlx prepare -- --lib

_check-sqlx-schema:
    cargo sqlx prepare --check -- --lib

# Release hooks
_release-prepare version:
    git-cliff --config configs/cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

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
