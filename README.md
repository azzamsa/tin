<div align="center">
  <h1>Rust GraphQL</h1>

<img src='docs/construction.svg' width=80px />

Rust GraphQL Template 🏗️

<a href="https://github.com/azzamsa/rust-graphql/workflows/ci.yml">
    <img src="https://github.com/azzamsa/rust-graphql/workflows/ci/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/rust-graphql">
    <img src="https://img.shields.io/crates/v/rust-graphql.svg">
  </a>

<a href=" https://docs.rs/rust-graphql/">
    <img src="https://docs.rs/rust-graphql/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

</div>

---

## Features

- [Async-GraphQL](https://github.com/async-graphql/async-graphql) GraphQL server library
  - Relay compatible cursor-based pagination
  - Playground disabled in the production environment for performance reasons
- [SQLx](https://github.com/launchbadge/sqlx) SQL toolkit
- [Axum](https://github.com/tokio-rs/axum) web framework
- [utoipa](https://github.com/juhaku/utoipa) Auto-generated OpenAPI documentation
- [git-cliff](https://github.com/orhun/git-cliff) Changelog Generator
- Exhaustive Integration tests
  - Uses [Cynic](https://github.com/obmarg/cynic) for GraphQL client
- Github Action for CI and release
- Git hooks for continuous development (format, lint, test)
  - Uses [Cargo Husky](https://github.com/rhysd/cargo-husky)
- Consistent formatting using [dprint](https://github.com/dprint/dprint)

## Usage

```bash
$ make setup

$ cargo r
```

## Development

```bash
$ clone the repository 

$ # Run unit tests and integration tests
$ make check

$ # Develop
$ make setup
$ make dev
```

## Credits

- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
