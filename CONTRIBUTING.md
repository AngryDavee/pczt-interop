# Contributing

This project follows the contribution standards of the Zcash Rust ecosystem. The
[`librustzcash` style guides](https://github.com/zcash/librustzcash/blob/main/CONTRIBUTING.md#styleguides)
are the primary reference for code style, and where this project contributes to existing
Zcash repositories it follows their
[merge](https://github.com/zcash/librustzcash/blob/main/CONTRIBUTING.md#merge-workflow),
[branch](https://github.com/zcash/librustzcash/blob/main/CONTRIBUTING.md#branch-history),
[pull request](https://github.com/zcash/librustzcash/blob/main/CONTRIBUTING.md#pull-request-review),
and [commit message](https://github.com/zcash/librustzcash/blob/main/CONTRIBUTING.md#commit-messages)
guidelines.

## Before opening a PR

- `cargo fmt` (rustfmt defaults).
- `cargo clippy --all-targets` with no new warnings.
- `cargo test` passes.

## Commit messages

Write clear, imperative commit messages ("Add X", not "Added X"). Keep the subject under
50 characters and explain the why in the body when it is not obvious.

## Scope

This repository is a proof of concept for PCZT cross-implementation interop testing. Issues
and PRs that extend the vectors, add a role adapter, or improve diagnostics are welcome.
