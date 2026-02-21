> [!TIP]
> Looking for something simpler? Try [Tun](https://github.com/azzamsa/tun).

<div align="center">

<img src='docs/logo.svg' width=250px />

<br/>
<h4>Rust GraphQL Template 🏗️</h4>

<a href="https://github.com/azzamsa/tin/actions/workflows/ci.yml">
    <img src="https://github.com/azzamsa/tin/actions/workflows/ci.yml/badge.svg" alt="Build status" />
  </a>

</div>

---

## Features

- [Async-GraphQL](https://github.com/async-graphql/async-graphql): GraphQL server library.
  - Supports Relay-compatible cursor-based pagination.
  - Automatically disables Playground in the production environment for performance reasons.
  - Automatically stores GraphQL schema to file.
- [SQLx](https://github.com/launchbadge/sqlx): SQL toolkit.
- [Axum](https://github.com/tokio-rs/axum): web framework.
- [Tracing](https://github.com/tokio-rs/tracing): includes local timestamps.
- [Frunk](https://github.com/lloydmeta/frunk): avoids writing repetitive boilerplate.
- [utoipa](https://github.com/juhaku/utoipa): automatically generates OpenAPI documentation.
- [git-cliff](https://github.com/orhun/git-cliff): Changelog Generator.
- Includes exhaustive integration tests.
  - Uses [Cynic](https://github.com/obmarg/cynic) as GraphQL client.
  - Uses [Hurl](https://github.com/Orange-OpenSource/hurl) for API collection.
- Utilizes fast and tiny image containers, leveraging [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) and `scratch` or `distroless` images.
- Implements GitHub Actions for CI and release workflows.
- Removed Git hooks for continuous development (formatting, linting, testing) [commit link](https://github.com/azzamsa/tin/commit/d9906164db7eb30cf66e2ed32edb220c0787fe13).
- Ensures consistent formatting using [dprint](https://github.com/dprint/dprint) for non-Rust files (Markdown, Dockerfiles, etc.).
- Supports [cargo-binstall](https://github.com/cargo-bins/cargo-binstall).
- Includes [cargo-release](https://github.com/crate-ci/cargo-release) workflow.

## Checklist

When you use this template, try to follow the checklist to update your info properly

- [ ] Change the author name in `LICENSE`
- [ ] Change the package info in `Cargo.toml`
- [ ] Change the application name:
  - [ ] Database name and other values in `.env`, `.example.env`, and other container related files.
  - [ ] The OpenAPI info in `routes.rs`
  - [ ] App name in `release.yml`
  - [ ] Project URL in `cliff.toml`
  - [ ] App name in the import statements across Rust source and tests files.
- [ ] Clean up the READMEs and remove routes

And, enjoy :)

## Usage

```shell
$ # Clone the repository

$ # Run the database
$ podman-compose up db -d

$ touch $SCHEMA_LOCATION # See .example.env
$ just dev  # See also `just setup`
```

Go to the playground `http://127.0.0.1:8000/playground` to see the schema.

## Navigating the Code

All the features can be found in the [CHANGELOG](CHANGELOG.md) file tagged with `feat`.
The file only contains user-facing changes, so you won't get lost navigating the code.

## Credits

- Clean and Scalable Architecture for Web Applications in Rust by Sylvain Kerkour. [Article](https://kerkour.com/rust-web-application-clean-architecture), [Code](https://github.com/skerkour/bloom-legacy/tree/v2-e2ee).
- Icons and emoji from [Noto Emoji](https://github.com/googlefonts/noto-emoji)
