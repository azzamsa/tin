.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	echo "::: Setting up..."
	git cliff --version || cargo install git-cliff
	sqlx --version || cargo install sqlx-cli --no-default-features --features postgres,native-tls
	cargo watch --version || cargo install cargo-watch
	cargo outdated --version || cargo install --locked cargo-outdated 
	npm install -g get-graphql-schema

dev:
	cargo watch -x clippy -x '+nightly fmt' -x run

c:
	cargo check

fmt: ## Format the codebase.
	cargo +nightly fmt

fmt_check: ## Check is the codebase properly formatted.
	cargo +nightly fmt --all -- --check

lint: ## Lint the codebase.
	cargo clippy --locked --all-targets

test:
	# Usage: make comply db_password=secret
	# Clean the db before the test
	env PGPASSWORD=$(db_password) psql --host localhost --username postgres nahla --command "DELETE FROM user_;"
	cargo test --all-targets

update:
	cargo update
	cargo outdated --root-deps-only

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

check: check_sqlx_schema fmt_check lint test ## Check if the repository comply with the rules and ready to be pushed.

store_schema: ## Update the schema
	get-graphql-schema http://127.0.0.1:8000/graphql > tests/schema.graphql

release:  ## Create a release
	bash scripts/release.sh $(version)