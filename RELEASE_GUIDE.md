# Release Guide

This is a monorepo which releases multiple crates to cargo.io and the Mantle CLI to GitHub releases for
installation via Foreman. Currently the release process is manual and is documented here.

## Commit Messages

Commit messages should be written using the [conventional
commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) format where the scope should be the name of
the crate which was updated. A single commit may affect multiple crates, in which case the scope should be a
comma-separated list.

## Crates

When a crate is ready for release, its version should be manually bumped according to the conventional commits
since the last release. If any other crates in the repo depend on that crate, they should be bumped as well.
In most cases dependent crates should be bumped a patch version, but use your judgement to determine if a
larger bump is required.

After bumping the crate, build it and commit the change. Ensure the build is passing in CI. Publish the crate
using the cargo CLI.

## Mantle CLI

When Mantle is ready for release, its crate should be updated following the above guide. To release the CLI,
add a git tag by running `git tag v<Version>`. After adding the tag, push it to GitHub with `git push --tags`.
A CI build will automatically run which will build the CLI for release and upload it to GitHub Releases. When
the release is created it will be in edit mode. Open it up and add a description. It can help to prefill the
message with the commit messages since the last release.

## Docs

When a new version of Mantle is released, the docs need to be updated to match. Open the Vercel project for
the docs site and trigger a redeployment of the `main` branch.
