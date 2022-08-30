# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2022-08-29

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

### Bug fixes

- `ServerParseError` in GraphQL playground ([9b80607](9b806076801c02eec2399c0210e27d05e15becb6))
- Migrate to axum ([3ab3468](3ab34682fec5b834376f4e8038e62cc8add7be55))
- Remove `unique hash` field ([ed4acd1](ed4acd1f00c4335b044e80b310ba7d6b1ffcc694))
- Refine struct visibility ([f57ee25](f57ee25b8ae6f59ed93043739b746329d4887490))
- Check existing username before update ([7d970d7](7d970d77b22494bfc938129b73d9ae91184c4a5f))
