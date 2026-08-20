# Week 1 Update

Status: submitted through Colosseum on August 17, 2026. The Eternal dashboard
marks the Week 1 video update complete.

## Objective

Freeze a safe, implementable agent-task contract and prove that the pinned
DeTrustPay devnet baseline is ready for SDK integration.

## Shipped

- Preserved the `pre-eternal-2026-h2` baseline and recorded canonical program
  provenance.
- Narrowed v0 to requester-created, single-capacity tasks settled in Circle
  devnet USDC.
- Defined one equal mutual deposit, constrained by the program to 25%–400% of
  payment.
- Defined content-addressed task and delivery-receipt formats, with the receipt
  pointer fitting the 128-byte participant-message limit.
- Added machine-readable task, receipt, and network schemas.
- Added external-signing security policy, stop-ship rules, and weekly acceptance
  gates.
- Implemented and deployed the incompatible version 2 program with snapshotted
  transaction policy, stable per-mint fee vaults, and complete Buyer/Seller
  `PartyOrder` mirrors for backend-independent retained-order queries.
- Added participant-owned rent: Buyer and Seller control and reclaim only their
  own order-copy SOL, while the shared vault refund is fixed to its payer.
- Removed automatic order, vault, and listing closure; cleanup is explicit and
  cannot be performed by management or a counterparty.
- Passed 16 Rust unit tests, a clean SBF build, a fresh local-validator lifecycle,
  and the same two-wallet lifecycle against the deployed devnet binary.

## Evidence

- Video: [DeTrustPay Eternal Week 1](https://www.youtube.com/watch?v=9L8WxJ_noGA)
- Recording script: [`week-1-video-script.md`](week-1-video-script.md)
- Baseline commit: `c19392a` (`pre-eternal-2026-h2`)
- Provenance commit: `cb556b5`
- Day 0 and Week 1 artifact checkpoint:
  [`b1f9cfa`](https://github.com/DeeKeenLab/detrustpay-agent/commit/b1f9cfa4399f74b9f77fce0d0a73e4d70142923d)
- Devnet program: `3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL`
- Initial deployment: `2zU61wsuJQnVgHnt8QZUgcoX4K5GXmXc69nVSpFPM1MgvFPjYD6oUiHhAcaDKGKp22G25tF2uicDyovfJYgQhU9V`
- Executable hash: `85d76f2458feaeed38cd4e8c89bbf5b51f6f090c5db926b89b4602479937a24b`
- Devnet lifecycle activity is team-controlled test evidence and remains
  excluded from external-user metrics.

## Feedback and learning

No external developer integration has been attempted yet.

Source and live-state review changed the design in five important ways:

1. The deployed program requires payer and payee deposits to be equal, so v0
   exposes one mutual-deposit value instead of two independent values.
2. Listing acceptance expiry is enforced on-chain, while delivery deadlines
   remain off-chain policy and must not be marketed as automatic settlement.
3. A stable program-controlled fee vault per mint prevents management changes
   from redirecting an already funded order's fee destination.
4. A real validator test caught a lamport-ordering bug that host tests and SBF
   compilation could not: direct rent reimbursement must occur after all CPIs.
5. A `PartyOrder` authority at fixed offset 8 lets public RPC return only a
   wallet's retained copies without a DeTrustPay backend or client-side scan.

## Metrics

External-user and automated lifecycle metrics remain zero at this draft stage.
They will not be combined or inflated with configuration reads, builds, or
team-controlled checks.

## Next priority

Implement the TypeScript SDK vertical slice—`createTask`, `searchTasks`,
`getTask`, `acceptTask`, `submitDelivery`, and `confirmDelivery`—then record the
first complete, externally signed devnet lifecycle.
