# Eternal Day 0 Readiness

Date: August 13, 2026

## Outcome

The Eternal product boundary and integration contract are frozen. The
authenticated dashboard shows that the sprint was already active before this
verification; this repository work did not start or modify the timer.

> Deployment update, August 13, 2026: the version 1 program audited below was
> subsequently closed. The fresh dual-order version 2 program is deployed at
> `3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL`, with Config PDA
> `CYrLYtpgk5UNuq3C7pjrYd6XuetTujL3Pu8JSk5ozSd8`. The table below is retained as
> historical Day 0 evidence, not current network configuration.

## Repository baseline

- Baseline tag: `pre-eternal-2026-h2`.
- Canonical program snapshot and IDL provenance are recorded in
  `program/UPSTREAM.md`.
- Agent work remains separate from the platform repository.
- The program snapshot is read-only integration input.

## Live devnet audit

Verified against Solana devnet on August 13, 2026:

| Item | Value |
|---|---|
| Program | `3QE69X6FsKxaop5osBW1WL11cCTMMvDuXJLZ9s2TcCNy` |
| Program executable | yes |
| Upgrade authority | `6YNNuEou8LQ1egLyYWJVZHmVsGHCzKMkA7tnJj4VzTe2` |
| Config PDA | `HMPQL5kXxmfGisbKREgGft533ASDPxUz7PBBjvPcTee6` |
| Config version | 2 |
| Program paused | no |
| Adjustable payments | enabled |
| Custom deposits | enabled |
| Dispute deterrent | disabled |
| Fee-vault authority | `6YNNuEou8LQ1egLyYWJVZHmVsGHCzKMkA7tnJj4VzTe2` |
| V0 settlement mint | Circle devnet USDC `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU` |
| Mint decimals | 6 |
| Token program | legacy SPL Token |

The program field named `fee_vault_account` is used as an authority. Settlement
instructions derive its associated token account for the order mint. Agent
documentation should call it the fee-vault authority to avoid implying that the
configured address itself must be an SPL token account. The pinned program
documentation is not edited here; a wording fix belongs in the canonical
platform repository.

## Frozen artifacts

- Network configuration: `config/devnet.json`
- Product decision: `docs/decisions/0001-eternal-v0-product-contract.md`
- Task and receipt contract: `docs/agent-task-v0.md`
- Machine-readable task schema: `packages/sdk/schema/task-v0.schema.json`
- Machine-readable receipt schema: `packages/sdk/schema/receipt-v0.schema.json`
- Delivery and release gates: `docs/eternal/acceptance-gates.md`

## Local verification

Verified on August 13, 2026 with Solana CLI 3.0.13, Anchor CLI 0.32.1,
Rust 1.90.0, and Solana platform tools 1.51:

- `make program-check`: passed.
- `make program-test`: passed, 7 tests and 0 failures.
- `make program-build`: completed and produced a 763 KiB SBF artifact with
  SHA-256 `2d97db99c878075012ea7abe15a95d674694578aa6a3962c25d19cd725f25811`.
- Network config and both schemas parse as JSON.
- `git diff --check`: passed.

The host check reports two existing Anchor macro `unexpected cfg(solana)`
warnings. SBF post-processing also reports unresolved syscall warnings for the
locally built artifact. The artifact is ignored and is not a deployment
candidate. Resolve the platform-tools warning in the canonical program
repository before any redeployment; it does not change the already deployed
devnet program used by the Eternal SDK.

## Competition portal

Verified read-only in the authenticated Colosseum dashboard on August 13, 2026:

- Eternal status: active; challenge already started.
- Week 1 video-update submission opens August 15.
- Final project submission opens September 1, 2026 at 12:44 AM UTC
  (August 31 at 9:44 PM Atlantic time).
- The submission-end countdown showed approximately 25 days and 4 hours. This
  implies a final deadline of September 8, 2026 at 12:44 AM UTC, subject to the
  live dashboard countdown remaining authoritative.
- Founder profile `JerryCa`: complete and ready.
- Project submission: not submitted, as expected before the final week.

The project review currently reports ten required fields missing:

- Project info: what is being built, why now, and technologies used.
- Media and code: project graphic, GitHub link, product demo, and pitch video.
- Team: primary location, Telegram contact, and accelerator application choice.

No timer, project, profile, weekly update, or submission control was changed
during verification.

## Day 0 checklist

- [x] Preserve and identify the pre-Eternal baseline.
- [x] Record canonical program provenance.
- [x] Confirm the deployed program and config are active on devnet.
- [x] Select the Eternal settlement mint and token program.
- [x] Freeze the v0 task schema and role mapping.
- [x] Freeze the compact delivery-receipt format.
- [x] Define external signing and policy boundaries.
- [x] Record the licensing direction for new agent components.
- [x] Define engineering and external-integration acceptance gates.
- [x] Confirm the submission window in the authenticated dashboard.
- [x] Confirm the Eternal timer is already active without changing it.

The immediate competition obligation is the Week 1 video update. Prepare its
one-minute Loom or YouTube URL before the August 15 submission window opens.
