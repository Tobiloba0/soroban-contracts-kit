#!/usr/bin/env bash
# Creates labels and seed issues using the GitHub CLI (gh).
# Usage: gh auth login && bash scripts/seed-issues.sh   (run from the repo root)
set -euo pipefail

gh label create "bug" --force >/dev/null
gh label create "ci" --force >/dev/null
gh label create "docs" --force >/dev/null
gh label create "enhancement" --force >/dev/null
gh label create "frontend" --force >/dev/null
gh label create "good first issue" --force >/dev/null
gh label create "new-contract" --force >/dev/null
gh label create "tests" --force >/dev/null

gh issue create --title "Add CI and license badges to README" --label "good first issue,docs" --body-file scripts/seed-issues/01.md
gh issue create --title "Document every public function in the vesting contract" --label "good first issue,docs" --body-file scripts/seed-issues/02.md
gh issue create --title "Add an escrow usage section with Stellar CLI examples" --label "good first issue,docs" --body-file scripts/seed-issues/03.md
gh issue create --title "Add a vesting usage section with Stellar CLI examples" --label "good first issue,docs" --body-file scripts/seed-issues/04.md
gh issue create --title "Add test: escrow rejects zero and negative amounts" --label "good first issue,tests" --body-file scripts/seed-issues/05.md
gh issue create --title "Add test: vesting rejects zero duration and zero total" --label "good first issue,tests" --body-file scripts/seed-issues/06.md
gh issue create --title "Add test: escrow beneficiary can refund" --label "good first issue,tests" --body-file scripts/seed-issues/07.md
gh issue create --title "Add CHANGELOG.md" --label "good first issue,docs" --body-file scripts/seed-issues/08.md
gh issue create --title "Add SECURITY.md" --label "good first issue,docs" --body-file scripts/seed-issues/09.md
gh issue create --title "Add CODE_OF_CONDUCT.md" --label "good first issue,docs" --body-file scripts/seed-issues/10.md
gh issue create --title "Add design notes for the escrow contract" --label "good first issue,docs" --body-file scripts/seed-issues/11.md
gh issue create --title "Add design notes for the vesting formula" --label "good first issue,docs" --body-file scripts/seed-issues/12.md
gh issue create --title "New contract: payment splitter" --label "enhancement,new-contract" --body-file scripts/seed-issues/13.md
gh issue create --title "Escrow: add an expiry after which the depositor can reclaim funds" --label "enhancement" --body-file scripts/seed-issues/14.md
gh issue create --title "Vesting: allow the funder to revoke and recover unvested tokens" --label "enhancement" --body-file scripts/seed-issues/15.md
gh issue create --title "Assert emitted events in tests" --label "tests" --body-file scripts/seed-issues/16.md
gh issue create --title "New contract: token faucet with per-address cooldown" --label "enhancement,new-contract" --body-file scripts/seed-issues/17.md
gh issue create --title "Extend instance storage TTL in escrow and vesting" --label "enhancement" --body-file scripts/seed-issues/18.md
gh issue create --title "CI: upload built wasm files and report their sizes" --label "ci" --body-file scripts/seed-issues/19.md
gh issue create --title "Workspace-level integration tests" --label "tests" --body-file scripts/seed-issues/20.md
gh issue create --title "New contract: M-of-N multisig treasury" --label "enhancement,new-contract" --body-file scripts/seed-issues/21.md
gh issue create --title "Property-based tests for vesting math" --label "tests" --body-file scripts/seed-issues/22.md
gh issue create --title "Vesting factory contract" --label "enhancement,new-contract" --body-file scripts/seed-issues/23.md
gh issue create --title "TypeScript bindings and a small demo web app" --label "enhancement,frontend" --body-file scripts/seed-issues/24.md
