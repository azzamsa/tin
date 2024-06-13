# Contributing Guide

## Development

```bash
$ # start the container database
$ podman-compose -f compose.local.yml up db -d

$ # start the backend in host
$ # you need to prepare the .env. Otherwise, start it in container. See "Configure Environment Variables"
$ just check
```

## Commit Message Format

This repo is using [Agular's commit message format][commit-message]

## Upgrading the Minimum Supported Rust Version (MSRV)

- Update the `rust-version` in `Cargo.toml`.
- Update the `channel` in `rust-toolchain.toml`.
- Update the Rust version of `cargo-chef` in the `Dockerfile`.

## Upgrading Dependencies

- Run `just up` to check for outdated dependencies, and then run `just up --write` to update them.
- Upgrade the `cargo-chef` version in the `Dockerfile`.

[commit-message]: https://github.com/angular/angular/blob/2095a08781167e91a60a4cec65c694688b319cd0/CONTRIBUTING.md#-commit-message-format
