## Release Checklist

- Ensure local `master` is up to date with respect to `origin/master`.
- Run `just up` and review semver incompatible updates.
- Run`just up --write` to upgrade dependencies.
- Run `just check`.
- Run the release task: `just release v<major.minor.path>`. Such `just release v0.1.7`.
- Check if [Continuous Integration][ci] workflow is completed successfully.
- Push the tags: `git push --tags`

<!-- dprint-ignore-start -->

[ci]: https://github.com/azzamsa/tin/actions/workflows/ci.yml

<!-- dprint-ignore-end -->
