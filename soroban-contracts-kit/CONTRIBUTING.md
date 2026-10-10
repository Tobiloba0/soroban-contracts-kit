# Contributing

Thanks for helping out! This guide keeps things smooth for everyone, especially during Wave sprints.

## Getting an issue

1. Browse open issues. Trivial ones are great first picks.
2. Comment on the issue explaining your approach in 2-3 sentences. Maintainers assign issues to contributors; please wait for assignment before starting.
3. One issue per pull request. Do not pick up more than two issues at once.
4. If you stop working on an issue, say so, so someone else can take it.

## Setup

```bash
rustup show            # installs the toolchain from rust-toolchain.toml
cargo test --workspace
```

## Before opening a PR

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

Then open a PR that includes `Closes #<issue-number>` and fill in the template.

## Code standards

- Every public contract function has a doc comment.
- Every new behaviour has a test, including failure cases.
- Use `contracterror` enums rather than panics for expected failures.
- Update state before making token transfers.
- Keep PRs small and focused.

## Review

Maintainers aim to review within 48 hours. During a Wave, reviews are prioritised so contributors can finish before the deadline. Be kind and constructive in reviews.
