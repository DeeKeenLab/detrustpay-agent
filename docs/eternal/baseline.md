# Pre-Eternal Baseline

Baseline date: August 13, 2026

This document records the state before the Colosseum Eternal timer is started.
It is intentionally candid so that sprint work can be evaluated against a
stable reference.

## Provenance

- Legacy repository: `https://github.com/JerryRenCA/TrustPay.git`
- Legacy branch at inspection: `autosave`
- Legacy HEAD: `1d826b3fb072900b2969383b34e88278b3179cd0`
- Legacy HEAD date: May 15, 2026
- Baseline import source: reviewed files from the current legacy working tree
- Intended repository: `DeeKeenLab/detrustpay-agent`

The legacy working tree contained extensive uncommitted work. This baseline is
a clean source snapshot rather than an assertion that every imported line was
created on the legacy HEAD date.

## What exists

- An Anchor program deployed on Solana devnet.
- Listing creation and acceptance.
- Payment and mutual-deposit locking in program-derived token vaults.
- Payer confirmation and payee cancellation paths.
- Adjustable-payment proposals and proposal acceptance.
- Participant message metadata.
- Lifecycle events for off-chain consumers.
- A generated program IDL.

Devnet program ID:

```text
3QE69X6FsKxaop5osBW1WL11cCTMMvDuXJLZ9s2TcCNy
```

## What does not exist in this repository

- A supported agent task schema.
- A standalone agent SDK.
- An MCP server.
- Reference requester and provider agents.
- A focused task explorer.
- An external developer integration.
- Meaningful real-user or agent usage.
- A published third-party security audit.
- A production or mainnet launch supported by this repository.

## Imported baseline files

- Anchor workspace and program Rust source.
- Current generated IDL used by the existing devnet tooling.
- Program behavior documentation.
- Existing source-availability license.

## Explicit exclusions

- Legacy `.git` history and object database.
- Human frontend, admin console, and generated static output.
- .NET backend, identity system, database migrations, and cloud deployment.
- Simulation bot and its generated wallets or state.
- Test keypairs, deploy keys, environment files, and secrets.
- Terraform variables, plans, state, and provider caches.
- Books and unrelated long-form content.

## Baseline tag

After review and before starting Eternal, create and push:

```sh
git tag pre-eternal-2026-h2
```

The pushed `pre-eternal-2026-h2` tag is the canonical baseline reference. Record
its resolved commit in the Colosseum submission notes before the competition
timer starts.
