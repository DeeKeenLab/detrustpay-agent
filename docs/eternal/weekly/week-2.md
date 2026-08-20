# Week 2 Update

Status: draft prepared August 20, 2026. The Colosseum dashboard enables the
Week 2 Loom or YouTube field on August 22.

## Objective

Remove the remaining browser dependency on the DeTrustPay backend and make
wallet recovery explicit, resumable, and safe for rate-limited public Solana
RPC endpoints.

## Shipped

- Replaced application identity with the connected Solana wallet for the
  payment workflow.
- Moved listing, order, message, proposal, and settlement writes to direct
  wallet-signed Solana transactions.
- Made on-chain accounts and transaction events canonical while retaining an
  IndexedDB mirror as a disposable local performance cache.
- Added a user-triggered wallet recovery flow instead of starting broad scans
  during page load.
- Limited active recovery to wallet-filtered account reads. Historical recovery
  fetches at most ten wallet signatures per user request and checks their
  transactions sequentially.
- Persisted recovery cursors so cancellation, reloads, and RPC 429 responses do
  not discard progress.
- Added request coalescing, a default concurrency limit of two, short-lived
  caching, and provider-wide exponential backoff for explicit RPC throttling.
- Published the portable policy and RPC coordination foundation in the agent
  SDK with automated tests.

## Evidence

- Week 2 video: `TBD — add the public Loom or YouTube URL after recording`
- Live devnet application: [detrustpay.com](https://detrustpay.com)
- Public repository: [DeeKeenLab/detrustpay-agent](https://github.com/DeeKeenLab/detrustpay-agent)
- Week 1 public checkpoint:
  [`b1f9cfa`](https://github.com/DeeKeenLab/detrustpay-agent/commit/b1f9cfa4399f74b9f77fce0d0a73e4d70142923d)
- Devnet program: [`3S3k...KcnL`](https://explorer.solana.com/address/3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL?cluster=devnet)
- Program deployment: [`2zU6...hU9V`](https://explorer.solana.com/tx/2zU61wsuJQnVgHnt8QZUgcoX4K5GXmXc69nVSpFPM1MgvFPjYD6oUiHhAcaDKGKp22G25tF2uicDyovfJYgQhU9V?cluster=devnet)
- SDK tests: policy boundaries, UTF-8 limits, request coalescing, cache
  invalidation, concurrency, and 429 retry behavior.

## Learning

Direct-chain correctness and RPC efficiency are separate problems. A browser
can remove its trusted backend and still overload a public RPC endpoint if it
automatically scans every account and transaction. DeTrustPay therefore shows
local state first and makes active sync, historical pages, and individual
record recovery explicit user actions.

The local database is not a second source of truth. A user can delete it and
rebuild their wallet projection from Solana in bounded steps.

## Metrics

All current devnet activity is team-controlled testing. External developers,
independent integrations, and external settled tasks remain zero until a
non-team tester completes the documented flow.

## Next priority

Move the direct-chain read and transaction construction surface into the agent
SDK, then run requester and provider examples with explicit external signing.
