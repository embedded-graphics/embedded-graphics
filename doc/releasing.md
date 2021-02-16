# Release process

Target audience: crate maintainers who wish to release `embedded-graphics` or `embedded-graphics-core`.

> Please take a cautious approach to this. If any step doesn't feel right or doesn't succeed smoothly, stop and rectify any issues before continuing.

## On GitHub

- Check that all desired PRs are merged and all desired issues are closed/resolved.
- Check that the latest master build passed in CircleCI.

## On your local machine

- `cd` to the repository root
- Use the crate MSRV of 1.40 by running `rustup override set 1.40`
- Check that `just` and `cargo-release` are installed and available in `$PATH`.
  - `just --version`
  - `cargo release --version`
- Ensure you have the latest changes with `git switch master` and `git pull`
- Check that your local repository is clean (no uncommitted changes, no unpushed commits. Ideally, use `git reset --hard origin/master` to ensure your local state is up to date with `origin/master`.
- Check that the crate version in `Cargo.toml` and `core/Cargo.toml` match the latest released versions on <https://crates.io/crates/embedded-graphics> and <https://crates.io/crates/embedded-graphics-core> respectively.
- Run `just build` to ensure the build passes locally.
  - If the build fails for any reason, stop the release process and fix any issues by creating PRs. The upstream master branch must remain the source of truth. Restart this checklist once `just build` passes.
- Double check the release level (major, minor, patch)
- Release the crates in the following order:
  1. For `embedded-graphics-core` **if there are changes to release**: `just release-core <level>`
  2. For `embedded-graphics`: `just release-e-g <level>`

  Where `<level>` is `major`, `minor`, `patch`, or a specific SemVer version number.

## Post release

- Check that the release commands pushed a Git tag for each crate published, something like `embedded-graphics-v0.7.0` or `embedded-graphics-core-v0.2.0`.
- For each crate's tag, go to its page at e.g. <https://github.com/embedded-graphics/embedded-graphics/releases/tag/embedded-graphics-v0.7.0-alpha.3>, click <kbd>Edit tag</kbd> and draft a release by:
  - Adding the crate name to the `Release title` field
  - Copying and pasting the latest released section out of the crate's `CHANGELOG.md` file into the `Describe this release` field. Do not include the version header, e.g.:

    ```markdown
    ### Added

    - [#111](https://github.com/embedded-graphics/embedded-graphics/pull/111) Added something

    ### Removed

    - [#222](https://github.com/embedded-graphics/embedded-graphics/pull/222) Removed a thing
    ```
  - Hit <kbd>Publish release</kbd>
  - Check that the release is displayed on the [repository homepage](https://github.com/embedded-graphics/embedded-graphics).
  - Post a link to the released tag (e.g. <https://github.com/embedded-graphics/embedded-graphics/releases/tag/embedded-graphics-v0.7.0-alpha.3>) to the embedded-graphics Matrix room at <https://matrix.to/#/!SfJCDXZbMHXkPovtKL:matrix.org>
  - If you are @jamwaffles, post a Tweet tagging @rustembedded with a happy announcement message.
