Soroban Contracts Kit

A collection of reusable smart contract components built with Rust and Soroban for the Stellar ecosystem.

Soroban Contracts Kit is an open-source project designed to provide a structured foundation for developing, testing, and maintaining smart contracts on Stellar. The project currently includes escrow and token vesting contracts, with an emphasis on clear interfaces, predictable contract behaviour, testing, and developer-friendly documentation.

«Status: Under active development. Features, interfaces, and documentation may evolve as the project progresses.»

Overview

Smart contracts are a core building block of decentralized applications. However, implementing common functionality safely requires careful handling of authorization, state transitions, time-based conditions, and unexpected inputs.

Soroban Contracts Kit aims to make these implementations easier to understand, test, and extend by organizing contract functionality into focused modules.

Current Modules

1. Escrow Contract

The escrow module provides a foundation for implementing smart-contract-based escrow workflows.

Potential use cases include:

- Holding assets until specified conditions are satisfied.
- Managing the parties involved in an escrow agreement.
- Enforcing authorized state transitions.
- Testing successful operations and failure scenarios.

The exact supported operations and security guarantees depend on the current contract implementation.

2. Vesting Contract

The vesting module provides a foundation for implementing time-based token vesting workflows.

Potential use cases include:

- Token allocation with time-based release conditions.
- Managing vesting schedules.
- Testing release conditions and boundary cases.
- Handling unauthorized operations and invalid inputs.

Refer to the contract source code and tests for the behaviour currently implemented.

Technology Stack

- Rust — smart contract implementation language.
- Soroban SDK — tools and APIs for Stellar smart contract development.
- Cargo — Rust package and workspace management.
- GitHub Actions — continuous integration and automated checks.

Project Structure

soroban-contracts-kit/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/
│       └── ci.yml
├── contracts/
│   ├── escrow/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── test.rs
│   └── vesting/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── test.rs
├── docs/
│   └── errors.md
├── scripts/
├── Cargo.toml
├── rust-toolchain.toml
├── CONTRIBUTING.md
├── LICENSE
└── README.md

Getting Started

Prerequisites

Install the following tools before working with the project:

- Git
- Rust toolchain
- Cargo
- A compatible Soroban SDK version

The project includes a "rust-toolchain.toml" file. Use it to identify the intended Rust toolchain version.

Clone the Repository

git clone https://github.com/Tobiloba0/soroban-contracts-kit.git
cd soroban-contracts-kit

Build the Workspace

From the repository root, run:

cargo build --workspace

Run Tests

Execute the workspace tests with:

cargo test --workspace

These commands run the build and test processes defined by the Rust workspace. If the build fails, check the installed Rust toolchain, dependency compatibility, and the error output.

Development Guidelines

Contributions should prioritize correctness, readability, testability, and security.

When modifying a contract:

1. Understand the existing contract interface and state model.
2. Identify the expected behaviour and relevant edge cases.
3. Implement the smallest change that addresses the requirement.
4. Add or update tests for success and failure scenarios.
5. Run the relevant tests and workspace checks.
6. Update documentation when behaviour or interfaces change.

Security-sensitive changes should receive careful review before being treated as production-ready.

Testing and Quality

Testing is an important part of maintaining reliable smart contracts.

Areas for continued test development include:

- Successful contract operations.
- Unauthorized access attempts.
- Invalid inputs and state transitions.
- Boundary conditions and time-dependent behaviour.
- Error handling and expected failure cases.
- Regression tests for fixed bugs.

The project includes a GitHub Actions workflow for continuous integration. Consult ".github/workflows/ci.yml" for the checks currently configured.

Contributing

Contributions, bug reports, documentation improvements, and feature proposals are welcome.

To contribute:

1. Review the "contribution guidelines" (CONTRIBUTING.md).
2. Check existing issues before starting new work.
3. Choose an issue or propose an improvement.
4. Create a focused branch for your changes.
5. Include tests and documentation where appropriate.
6. Submit a pull request describing the change and how it was tested.

For larger changes, open an issue first to discuss the approach and avoid duplicating work.

Roadmap

Potential areas for future development include:

- Expanding escrow and vesting test coverage.
- Improving security-focused test scenarios.
- Documenting contract interfaces and error codes.
- Adding examples for common integration workflows.
- Improving continuous integration and contributor onboarding.
- Introducing additional reusable Soroban contract components as requirements emerge.

Roadmap items are proposals, not guarantees of implementation.

Security

Smart contracts can manage valuable assets, and defects may have serious consequences.

- Do not assume contracts are production-ready without an appropriate review.
- Test changes thoroughly before deployment.
- Never include private keys, seed phrases, or other secrets in source code.
- Report suspected security vulnerabilities responsibly rather than publishing exploit details.

No independent security audit is claimed by this repository unless explicitly documented.

Documentation

- "Error Reference" (docs/errors.md)
- "Contribution Guidelines" (CONTRIBUTING.md)
- "GitHub Issues" (https://github.com/Tobiloba0/soroban-contracts-kit/issues)
- "GitHub Repository" (https://github.com/Tobiloba0/soroban-contracts-kit)

License

This project is distributed under the license specified in the ""LICENSE"" (LICENSE) file. Review that file for the applicable permissions and conditions.

---

Built with Rust and Soroban for the Stellar ecosystem.

Contributions and constructive feedback are welcome.
