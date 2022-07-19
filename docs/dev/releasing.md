## Release Checklist

- Run the lint check: `make check`.
- Run the release task: `make release version=v<mayor.minor.path>`. Such `make release version=v0.1.7`.
- Push the release commit.
- Check if [Continuous Integration](https://github.com/azzamsa/rust-graphql/actions/workflows/ci.yml) workflow is completed successfully.
- Push the release tags.
- Wait for [Continuous Deployment](https://github.com/azzamsa/rust-graphql/actions/workflows/cd.yml) workflow to finish.
- Create a new GitHub release with the created tag above, and copy the release news from the CHANGELOG.md.
  Document Title
