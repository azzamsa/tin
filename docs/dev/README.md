# Contributing Guide

## Development

```bash
$ # start the container database
$ podman-compose -f docker-compose.local.yml up db -d

$ # start the backend in host
$ # you need to prepare the .env. Otherwise, start it in container. See "Configure Environment Variables"
$ just check
```

## Commit Message Format

This repo is using [Agular's commit message format][commit-message]

[commit-message]: https://github.com/angular/angular/blob/2095a08781167e91a60a4cec65c694688b319cd0/CONTRIBUTING.md#-commit-message-format
