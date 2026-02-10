#!/usr/bin/env -S just --justfile

set dotenv-load := true

alias d := dev
alias f := fmt
alias l := lint
alias t := test
alias c := comply
alias k := check

[doc('List available commands')]
_default:
    just --list --unsorted

[confirm('⚠️ This command will alter your system. Run recipe `setup`?')]
[doc('Setup the repository')]
setup:
    cargo binstall cargo-edit cargo-nextest cargo-outdated dprint git-cliff bacon typos-cli
    cargo binstall sqlx-cli

[doc('Tasks to make the code-base comply with the rules. Mostly used in git hooks')]
comply: _doc-check _update-sqlx-schema fmt lint test

[doc('Check if the repository comply with the rules and ready to be pushed')]
check: _doc-check _check-sqlx-schema fmt-check lint test

[doc('Develop the app')]
dev:
    bacon

[doc('Build the app')]
build:
    BUILD_HASH=$(git rev-parse --short=12 HEAD) BUILD_TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ) cargo build --release

[doc('Build the container image')]
image-build:
    podman build -t tin:latest --build-arg BUILD_HASH=$(git rev-parse --short HEAD) --build-arg BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ) .

[doc('Run the container')]
image-start service='':
    podman compose up {{ service }} -d

[doc('Stop the container')]
image-stop:
    podman compose down

[doc('Restart the containers')]
image-restart:
    just image-stop
    just image-run

[doc('Format the codebase.')]
fmt:
    cargo fmt --all
    dprint fmt
    hurlfmt tests/api-collection/**/*.hurl --in-place

[doc('Check is the codebase properly formatted')]
fmt-check:
    cargo fmt --all -- --check
    dprint check
    hurlfmt tests/api-collection/**/*.hurl --check

[doc('Lint the codebase')]
lint:
    cargo clippy --all-targets --all-features
    typos

[doc('Test the codebase')]
test:
    cargo nextest run --config-file .nextest.toml

[doc('Run the unit tests')]
test-unit:
    cargo nextest run --lib

[doc('Run the unit tests')]
test-api:
    ./tests/api-collection/test.sh

[doc('Create a new release. Example `cargo-release release minor --tag-name v0.2.0`')]
release level:
    cargo-release release {{ level }} --execute

[doc('Make sure the repo is ready for release')]
release-check level: check
    just up
    cargo-release release {{ level }}

[doc('Setup the database schema.')]
db-migrate:
    sqlx database create
    # sqlx migrate run # auto migration enabled

[doc('Reset the database schema')]
db-reset:
    sqlx database drop && sqlx database create

[doc('Check the documentation')]
_doc-check:
    cargo doc --all-features --no-deps

[doc('Update the SQLx schema')]
_update-sqlx-schema:
    cargo sqlx prepare -- --lib

[doc('Check the SQLx schema')]
_check-sqlx-schema:
    cargo sqlx prepare --check -- --lib

[doc('Prepare release hooks')]
_release-prepare version:
    git-cliff --config .cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

[doc('Check dependencies health. Pass `--write` to upgrade dependencies')]
up arg="":
    if [ "{{ arg }}" = "--write" ]; then \
        cargo upgrade --incompatible --recursive --verbose && \
        cargo update && \
        dprint config update; \
    else \
        cargo outdated --root-deps-only; \
    fi

[doc('Dependency analysis')]
meta:
    cargo +nightly udeps
    cargo audit
