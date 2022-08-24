.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	echo "::: Setting up..."
	git cliff --version || cargo install git-cliff
	sqlx --version || cargo install sqlx-cli --no-default-features --features postgres,native-tls
	cargo watch --version || cargo install cargo-watch
	cargo outdated --version || cargo install --locked cargo-outdated
	dprint --version || cargo install dprint
	## npm install -g get-graphql-schema

dev:
	cargo watch -x clippy -x '+nightly fmt' -x run

fmt: ## Format the codebase.
	cargo +nightly fmt --all
	dprint fmt --config configs/dprint.json

fmt_check: ## Check is the codebase properly formatted.
	cargo +nightly fmt --all -- --check
	dprint check --config configs/dprint.json

lint: ## Lint the codebase.
	cargo clippy --locked --all-targets --all-features

doc_check: ## Check the documentation.
	cargo doc --all-features --no-deps

test: ## Test the codebase.
	cargo test --all-targets -- --test-threads 1

update_sqlx_schema:
	cargo sqlx prepare -- --lib

check_sqlx_schema:
	cargo sqlx prepare --check -- --lib

migrate_db: ## Setup the database schema.
	sqlx database create
	sqlx migrate run

reset_db: ## reset the database schema.
	sqlx database drop && sqlx database create

comply: fmt lint test update_sqlx_schema ## Tasks to make the code-base comply with the rules. Mostly used in git hooks.

check: check_sqlx_schema fmt_check lint test doc_check ## Check if the repository comply with the rules and ready to be pushed.

update_graphql_schema: ## Update the schema
	## run the app before running this command `cargo r`
	get-graphql-schema http://127.0.0.1:8000/graphql > tests/schema.graphql

release:  ## Create a release
	bash scripts/release.sh $(version)

#
# Misc
#

check_dependencies: ## Check dependencies health.
	cargo +nightly udeps
	cargo outdated --root-deps-only
