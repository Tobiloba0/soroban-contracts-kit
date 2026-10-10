#!/usr/bin/env python3
"""Generate seed issues. Run from repo root: python3 scripts/issues.py"""

T, M, H = "Trivial", "Medium", "High"

ISSUES = [
 (T, "Add CI and license badges to README", "good first issue,docs",
  "Add a CI status badge and keep the license badge in README.md.",
  ["Badges render on GitHub", "Links point to the correct workflow"]),
 (T, "Document every public function in the vesting contract", "good first issue,docs",
  "Add `///` doc comments with parameter and error descriptions to each public function in `contracts/vesting`.",
  ["All public functions documented", "`cargo doc` has no warnings"]),
 (T, "Add an escrow usage section with Stellar CLI examples", "good first issue,docs",
  "Add `contracts/escrow/README.md` showing how to deploy, init, release and refund using the `stellar` CLI.",
  ["Commands are copy-pasteable", "Explains testnet setup briefly"]),
 (T, "Add a vesting usage section with Stellar CLI examples", "good first issue,docs",
  "Add `contracts/vesting/README.md` with deploy, init, claim and view examples.",
  ["Commands are copy-pasteable", "Explains cliff and duration with an example"]),
 (T, "Add test: escrow rejects zero and negative amounts", "good first issue,tests",
  "Add tests to `contracts/escrow/src/test.rs` asserting `InvalidAmount` for 0 and -1.",
  ["Both cases covered", "`cargo test` passes"]),
 (T, "Add test: vesting rejects zero duration and zero total", "good first issue,tests",
  "Add tests asserting `InvalidSchedule` for duration 0 and `InvalidAmount` for total 0.",
  ["Both cases covered", "`cargo test` passes"]),
 (T, "Add test: escrow beneficiary can refund", "good first issue,tests",
  "Existing tests cover arbiter refunds. Add one where the beneficiary refunds the depositor.",
  ["Balances asserted", "Status asserted"]),
 (T, "Add CHANGELOG.md", "good first issue,docs",
  "Create CHANGELOG.md following the Keep a Changelog format with an initial 0.1.0 entry.",
  ["Format follows keepachangelog.com"]),
 (T, "Add SECURITY.md", "good first issue,docs",
  "Add a security policy explaining how to privately report vulnerabilities and that the code is unaudited.",
  ["Contact method described", "Unaudited warning included"]),
 (T, "Add CODE_OF_CONDUCT.md", "good first issue,docs",
  "Add the Contributor Covenant v2.1 and link it from CONTRIBUTING.md.",
  ["File added", "Linked from CONTRIBUTING.md"]),
 (T, "Add design notes for the escrow contract", "good first issue,docs",
  "Create `docs/escrow-design.md` explaining roles, state machine (Funded -> Released/Refunded) and why state is updated before transfers.",
  ["Includes a state diagram (mermaid or ASCII)", "Explains trust assumptions"]),
 (T, "Add design notes for the vesting formula", "good first issue,docs",
  "Create `docs/vesting-design.md` explaining the cliff and linear formula with worked examples.",
  ["At least 3 worked examples", "Mentions rounding behaviour"]),
 (M, "New contract: payment splitter", "enhancement,new-contract",
  "Create `contracts/splitter`: accepts a token and distributes it between recipients according to shares (basis points that sum to 10,000).",
  ["init validates shares sum to 10,000", "`distribute` pays each recipient proportionally", "Rounding dust is handled and documented", "Full tests including failure cases"]),
 (M, "Escrow: add an expiry after which the depositor can reclaim funds", "enhancement",
  "Add an `expiry` timestamp to the escrow config. After expiry, the depositor can call `reclaim` without the arbiter.",
  ["New error for early reclaim", "Tests use ledger timestamp", "Docs updated"]),
 (M, "Vesting: allow the funder to revoke and recover unvested tokens", "enhancement",
  "Add an optional revoker set at init. `revoke` sends unvested tokens back to the revoker and freezes the schedule at the current vested amount.",
  ["Beneficiary keeps already vested tokens", "Cannot revoke twice", "Tests for all paths"]),
 (M, "Assert emitted events in tests", "tests",
  "Add test assertions for the events published by escrow and vesting using `env.events().all()`.",
  ["Each event type asserted at least once"]),
 (M, "New contract: token faucet with per-address cooldown", "enhancement,new-contract",
  "Create `contracts/faucet` that dispenses a fixed amount per request, with a cooldown between requests per address.",
  ["Cooldown enforced via ledger timestamp", "Admin can fund and set amount", "Tests included"]),
 (M, "Extend instance storage TTL in escrow and vesting", "enhancement",
  "Call `extend_ttl` on instance storage during state-changing calls so contracts don't expire unexpectedly.",
  ["Constants for thresholds defined", "Test verifies TTL is extended"]),
 (M, "CI: upload built wasm files and report their sizes", "ci",
  "Extend the CI workflow to upload wasm artifacts and print each file's size in the job summary.",
  ["Artifacts downloadable from a run", "Sizes visible in summary"]),
 (M, "Workspace-level integration tests", "tests",
  "Add a `tests/` crate that deploys escrow and vesting together, e.g. an escrow whose beneficiary is a vesting contract.",
  ["Runs in `cargo test --workspace`", "Documented in CONTRIBUTING.md"]),
 (H, "New contract: M-of-N multisig treasury", "enhancement,new-contract",
  "Create `contracts/multisig`: signers propose token transfers, and a transfer executes once M signers approve.",
  ["Propose, approve, execute, cancel flows", "Signers can't double-approve", "Threshold validated at init", "Thorough tests"]),
 (H, "Property-based tests for vesting math", "tests",
  "Use `proptest` to check invariants: vested is monotonic, never exceeds total, equals total at the end, claims never exceed vested.",
  ["At least 4 properties", "Runs in CI in reasonable time"]),
 (H, "Vesting factory contract", "enhancement,new-contract",
  "Create a factory that deploys a new vesting contract per schedule and keeps a registry of schedules per beneficiary.",
  ["Deploys from uploaded wasm hash", "Query schedules by beneficiary", "Integration tests"]),
 (H, "TypeScript bindings and a small demo web app", "enhancement,frontend",
  "Generate TypeScript bindings with `stellar contract bindings typescript` and build a minimal demo (escrow create/release) on testnet.",
  ["README with setup steps", "Works with a Freighter wallet on testnet", "No secrets committed"]),
]

import os, json
os.makedirs("scripts/seed-issues", exist_ok=True)
lines = ["#!/usr/bin/env bash",
 "# Creates labels and seed issues using the GitHub CLI (gh).",
 "# Usage: gh auth login && bash scripts/seed-issues.sh   (run from the repo root)",
 "set -euo pipefail", ""]
labels = sorted({l for i in ISSUES for l in i[2].split(",")} | {"bug", "enhancement"})
for l in labels:
    lines.append(f'gh label create "{l}" --force >/dev/null')
lines.append("")
for n, (cx, title, labs, desc, crit) in enumerate(ISSUES, 1):
    body = f"**Complexity:** {cx}\n\n{desc}\n\n**Acceptance criteria**\n" + "\n".join(f"- [ ] {c}" for c in crit)
    body += "\n\nSee CONTRIBUTING.md for the workflow.\n"
    path = f"scripts/seed-issues/{n:02d}.md"
    open(path, "w").write(body)
    lines.append(f"gh issue create --title {json.dumps(title)} --label {json.dumps(labs)} --body-file {path}")
open("scripts/seed-issues.sh", "w").write("\n".join(lines) + "\n")
