# soroban-contracts-kit

Small, well-tested, reusable [Soroban](https://developers.stellar.org/docs/build/smart-contracts) smart contracts for the Stellar network. Copy them, learn from them, or extend them.

![License](https://img.shields.io/badge/license-MIT-blue)

## Contracts

| Contract | What it does | Status |
|---|---|---|
| [`escrow`](contracts/escrow) | Three-party token escrow (depositor, beneficiary, arbiter) | Ready |
| [`vesting`](contracts/vesting) | Linear token vesting with a cliff | Ready |
| payment splitter | Split incoming tokens between recipients by share | Open issue |
| multisig treasury | M-of-N approved token transfers | Open issue |

## Quick start

```bash
git clone https://github.com/Tobiloba0/soroban-contracts-kit.git
cd soroban-contracts-kit

# install Rust: https://rustup.rs  (rust-toolchain.toml adds the right target)
cargo test --workspace
cargo build --release --target wasm32v1-none
```

Built wasm files land in `target/wasm32v1-none/release/`.

## Project layout

```
contracts/
  escrow/     # lib.rs (contract) + test.rs (tests)
  vesting/
docs/         # errors, design notes
scripts/      # maintainer helpers
```

## Contributing

We welcome first-time contributors. Read [CONTRIBUTING.md](CONTRIBUTING.md), then pick an issue labelled `good first issue`. This repo is designed to be run through [Drips Wave](https://www.drips.network/wave) sprints, so issues are small and clearly scoped.

## Security

These contracts are educational reference code and have **not been audited**. Do not use them with real funds without your own review.

## License

MIT
