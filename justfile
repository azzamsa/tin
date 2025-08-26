#!/usr/bin/env -S just --justfile

set dotenv-load := true

alias d := dev
alias r := run
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
    just _cargo-install 'cargo-edit cargo-nextest cargo-outdated dprint git-cliff bacon typos-cli'
    just _cargo-install 'sqlx-cli'

[doc('Tasks to make the code-base comply with the rules. Mostly used in git hooks')]
comply: _doc-check _update-sqlx-schema fmt lint test

[doc('Check if the repository comply with the rules and ready to be pushed')]
check: _doc-check _check-sqlx-schema fmt-check lint test

[doc('Develop the app')]
dev:
    bacon

[doc('Run the app')]
run:
    cargo run

[doc('Build the container image')]
image-build:
    docker build . --tag tin:latest --build-arg VCS_REVISION=$(git rev-parse --short HEAD)

[doc('Run the container')]
image-start service='':
    docker compose --file compose.local.yml up {{ service }} -d

[doc('Stop the container')]
image-stop:
    docker compose --file compose.local.yml down

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
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade --incompatible --recursive --verbose
        cargo update
        dprint config update
    else
        cargo outdated --root-deps-only
    fi;

#
# Helper
#

[doc('Install using plain cargo or cargo-binstall')]
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
