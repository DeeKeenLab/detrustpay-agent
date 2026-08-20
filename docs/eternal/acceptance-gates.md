# Eternal Acceptance Gates

These are release gates, not aspirational metrics. A week is not complete until
its required evidence is linked from the corresponding weekly update.

## Day 0: ready to start

- Baseline tag resolves and program provenance is reproducible.
- `config/devnet.json` matches the deployed program, config PDA, and selected
  settlement mint.
- Task and receipt schemas parse as JSON and the written contract matches them.
- External-signing, devnet-only, and off-chain-verification boundaries are
  explicit.
- Program host check and unit tests pass from the competition branch.
- Eternal dashboard deadline is confirmed before the timer is started.

## Week 1: fully on-chain order state

- A fresh version 2 program is deployed and initialized on Solana devnet.
- Every accepted order has synchronized Buyer and Seller `PartyOrder` accounts.
- Each participant controls and can reclaim only their own retained-copy rent.
- Wallet-filtered public RPC can discover retained orders without a backend.
- Unit, local-validator, and two-wallet devnet lifecycle checks pass.
- The one-minute update links the deployed program and describes the actual
  shipped boundary without claiming off-chain verification.

## Week 2: backendless browser recovery

- Wallet identity replaces backend login for the payment workflow.
- Core payment writes are direct, wallet-signed Solana transactions.
- Local data is a disposable mirror that can be rebuilt from Solana.
- Active account recovery uses wallet filters; history loads at most ten
  signatures per explicit user action and persists its cursor.
- Identical RPC reads are coalesced, concurrency is capped, and 429 responses
  trigger shared exponential backoff instead of parallel retry storms.
- A clean Node 20 installation builds and tests the public SDK foundation.
- The public repository passes its sensitive-path, secret-pattern, JSON, and
  oversized-file checks.

## Week 3: agent transaction vertical slice

- SDK rejects unsafe numbers and exposes token amounts as `bigint`.
- Spending, deposit, mint, program, cluster, counterparty, and deadline policies
  fail closed.
- `createTask`, `searchTasks`, `getTask`, `acceptTask`, `submitDelivery`, and
  `confirmDelivery` work through public SDK APIs.
- Every write returns an unsigned transaction or explicit sign request.
- Requester and provider examples run as separate processes.
- MCP tools expose bounded reads and transaction preparation without a private
  key or seed-phrase input.
- At least one complete externally signed devnet lifecycle has recorded
  signatures.

## Week 4: submission ready

- A clean-machine rehearsal reaches a settled devnet task in under 20 minutes.
- No secret, keypair, wallet directory, or environment file is tracked.
- All lifecycle and negative-path tests pass in CI.
- Security and protocol limitations are visible in the README and demo.
- The pitch and technical walkthrough use real devnet evidence.
- A developer can follow the quickstart without team intervention.
- External feedback and independent integration attempts are documented without
  combining them with team-controlled activity.
- Weekly updates, metrics, transaction links, feedback, and final repository
  revision are complete.

## Stop-ship conditions

- Any component accepts or logs private-key material.
- The SDK can silently switch cluster, program, or mint.
- A manifest mismatch is presented as a valid task.
- Marketing says the program verified an off-chain artifact or deadline.
- The demonstration depends on a team-only uncommitted service or keypair.
- External traffic is mixed with simulations or team-controlled wallets.
- Mainnet or real-value use is implied without a published security audit.
