# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2023-07-19

### Features

- Min sized binary ([198afea](198afea463ad3a069c7519c54cf32f582906162f))

## [0.4.0] - 2023-07-19

### Features

- Avoid boilerplate with `frunk` ([05a920b](05a920bd38715746fbe46c2a15a7ce5374b24597))
- Containerize app ([7a0b138](7a0b138520b535877b75e036bd50b77302036e78))

### Bug fixes

- Git-hooks should run only in CI ([d990616](d9906164db7eb30cf66e2ed32edb220c0787fe13))

  It slows down the development.

## [0.3.2] - 2023-01-04

### Bug fixes

- Migrate to `tokio` runtime ([6a83302](6a833026e333129c9eb33a56d8b5d6e1e8984c1b))

## [0.3.1] - 2022-12-24

### Bug fixes

- Avoid checking if the config exists ([fdb8796](fdb8796958f91a3de4f7ca6d794e5099048e86e2))

  This eliminates a [race condition between "check to see if file exists" and "open file if it exists"](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use).

## [0.3.0] - 2022-11-08

### Features

- Use local timestamp in tracing ([2ecaaaf](2ecaaaf9eb461007dc21fbdc154c86bebd569fca))

### Bug fixes

- Migrate to tracing ([8d9710d](8d9710d83329f4286f32efafcc3db3d03babc67a))

  - local timestamp support
    - log function name support

## [0.2.1] - 2022-10-16

### Bug fixes

- `Cargo.lock` version is outdated ([913596c](913596c4db6c2357492b49766189204dcb9a7b76))

## [0.2.0] - 2022-10-16

### Features

- Make writing schema optional ([f81aebb](f81aebb5a30007aa0b5bed1a6fb660eb1e42789e))

## [0.1.2] - 2022-08-30

### Features

- Store GraphQL schema to file automatically ([1a9b985](1a9b985c0a58baf5b796ec3ce20080d9316f269b))

### Bug fixes

- Validate GrahpQL schema location ([d7bdc52](d7bdc523789a0b7b24f8ebe84119f821176c619a))

## [0.1.1] - 2022-07-22

### Features

- Async pagination ([e1ca0fd](e1ca0fd4f3371eef0dd7eadbf189e7eebb93661c))
- Total count in pagination ([f0d8374](f0d837416b37647f3ea1cd59331f3ba6c9f17314))
- Base64 cursor ([5eb1d93](5eb1d9374236aa37d38b8ad2450c7f0f720959bd))
- In-house relay like pagination ([fc2d743](fc2d74324c68ef63b74960cdcfafe8af4bdb6099))
- Basic pagination for users ([c109bc9](c109bc9df0a548b84323d9342596235f8d4ffdcc))
- OpenAPI documentation ([e8ecff1](e8ecff1b81e2843523a8d551f92ab1a5b66dc348))
- Stand-alone health REST endpoint ([3988721](39887214e948e6d5fb6f1a22b058da0bd59d471e))
- Disable GraphQL playground in production ([b8b43ec](b8b43ece8a59aefe1de6fbc81c4f7b1c3c980d9a))

  For performance reason.

### Bug fixes

- `ServerParseError` in GraphQL playground ([9b80607](9b806076801c02eec2399c0210e27d05e15becb6))
- Migrate to axum ([3ab3468](3ab34682fec5b834376f4e8038e62cc8add7be55))
- Remove `unique hash` field ([ed4acd1](ed4acd1f00c4335b044e80b310ba7d6b1ffcc694))

  Demonstrate the strength of kerkour architecture later using a real model.

- Refine struct visibility ([f57ee25](f57ee25b8ae6f59ed93043739b746329d4887490))
- Check existing username before update ([7d970d7](7d970d77b22494bfc938129b73d9ae91184c4a5f))
