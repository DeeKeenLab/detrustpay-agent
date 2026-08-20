# Program Overview

This document describes what the DeTrustPay Solana program does at a user and
integrator level. It is not a license grant and is not a security audit.

This document describes the breaking version 2 source. Version 2 requires a
fresh program deployment and Config PDA; it is not an upgrade path for funded
version 1 accounts.

## Purpose

DeTrustPay is a structured settlement protocol for promise-based transactions.
It turns an ordinary Promise into a Structured Promise by placing it inside
defined terms, mutual economic exposure, recognized actions, deadlines, and
predefined settlement consequences.

The core mechanism is Mutual Economic Exposure, or MEE: both parties lock
economically meaningful value before a vulnerable transaction stage begins. The
protocol extends base double-deposit logic through eDDE, an enhanced
dispute-convergence model that treats confirmation, refusal, proposals, silence,
deadlines, and terminal outcomes as recognized transaction actions.

DeTrustPay does not attempt to directly judge all external performance. Instead,
the Solana program controls the settlement layer around off-chain Promises by
defining locked value, valid actions, state transitions, and settlement
consequences. A user can publish reusable terms, another user can accept those
terms into a specific order, and the program holds both sides' locked token
amounts in PDA vaults until the order reaches a recognized settlement outcome.

The version 2 source supports SPL-compatible token accounts through Anchor's
token interface. SOL-denominated flows are represented through wrapped SOL token
accounts.

## Main Ideas

- A `Listing` is a reusable Structured Promise template: it defines the payment
  amount, payer and payee deposits, accepted counterparty rules, capacity, and
  optional expiration.
- A `PartyOrder` is one participant-owned mirror of an accepted Structured
  Promise. Every accepted order has a Buyer copy and a Seller copy.
- MEE is implemented by locking the payer side's payment/deposit exposure and
  the payee side's deposit exposure in program-derived token vaults.
- eDDE is implemented through recognized on-chain actions: accept, confirm,
  cancel, propose adjusted terms, accept a proposal, attach participant
  messages, close eligible state, and emit lifecycle events.
- Deadlines are represented where supported by listing expiration and proposal
  expiry fields. Frontends and indexers can use account state and events to make
  silence and deadline status visible to users.
- The payer and payee can attach short plaintext or encrypted metadata to an
  order, but encryption and business interpretation happen off-chain.
- The program emits events for indexers and frontends to track the listing and
  order lifecycle.

## Protocol Boundary

The program is the settlement layer, not an external-performance oracle. It can
enforce who may act, what funds are locked, which actions are valid in the
current state, how proposals affect settlement, and where tokens move at a
terminal outcome. It does not decide whether goods were delivered, services were
performed, or an off-chain Promise was morally or legally satisfied. Those
judgments remain outside the program and are reflected only through participant
actions.

## Accounts

### Config

The singleton `Config` account stores protocol settings:

- `manage_authority`: authority allowed to update program configuration.
- `enable_adjustable_payment`: enables proposal-based payment adjustment.
- `enable_custom_deposit`: enables custom payer/payee deposit amounts.
- `enable_dispute_deterrent`: selects proposal-related deterrent behavior for
  newly created listings.
- `paused` and `paused_at_slot`: stop new exposure while preserving existing
  order settlement paths.

The configuration has no legacy version field or migration path. This account
layout belongs to a new, intentionally incompatible program deployment.

Protocol fees are held in one deterministic token-vault PDA per mint, derived
from `protocol_fee_vault` and the mint address. The Config PDA is the token
authority. The manage authority may withdraw accumulated fees but cannot
change the vault used by a funded order.

### Listing

A `Listing` stores reusable Structured Promise terms:

- `id`: 16-byte listing identifier.
- `title` and `description`: short display metadata.
- `creator`: user who created the listing.
- `is_payer_listing`: whether the creator is the payer side.
- `counterparty`: optional restricted counterparty; default means open listing.
- `mint_account` and `mint_decimals`: token mint used for payment.
- `creator_token_account`: creator's token account for this mint.
- `payment_amount`: base payment amount.
- `payer_deposit_amount` and `payee_deposit_amount`: locked deposits.
- `accept_capacity`: maximum accepted order count.
- `used_capacity`: consumed capacity.
- `active_orders`: accepted orders not yet settled or cancelled.
- `next_order_index`: monotonic index used to derive order PDAs.
- `revision`: monotonic listing state revision.
- `category`: fixed-width marketplace category used by RPC filters.
- `listing_token_vault_account`: PDA token vault holding the creator side's
  locked amount for future accepts.
- `creator_order_rent_reserve`: creator-funded SOL reserved for the creator's
  future order copies.
- `listing_vault_closed`: records explicit listing-vault cleanup.
- `expiration`: optional timestamp after which the listing cannot be accepted.
- `dispute_deterrent_enabled`: immutable snapshot of the global setting when
  the listing is created.

### PartyOrder

Acceptance creates two complete `PartyOrder` accounts for the same order:

- Buyer copy: authority is the payer.
- Seller copy: authority is the payee.

The fixed prefix stores `authority` at raw account-data offset 8,
`counterparty`, `role`, a digest of the complete shared order payload, and the
copy PDA bump. The shared payload stores the order ID, listing identity,
participants, accepted economic and dispute rules, mint and token destinations,
order vault, proposal/message state, revision, dates, terminal state, and
explicit vault-cleanup state.

Every active lifecycle instruction validates both roles, reciprocal authorities,
identical payloads, and recomputed digests. It applies the transition to the
Buyer copy and synchronizes the complete result and digest to the Seller copy in
the same Solana transaction.

A wallet discovers its retained copies with one server-side filtered
`getProgramAccounts` request using the `PartyOrder` discriminator and a memcmp at
authority offset 8. It does not download every program account and does not need
a DeTrustPay backend index. Local IndexedDB is only a restorable cache of this
chain state.

The two state accounts are not token-vault authorities. The vault is controlled
by a separate accountless `OrderAuthority` PDA, so either participant can later
close their own state copy without affecting token authority or the other copy.

## Core Flows

### 1. Initialize Configuration

`initialize_config` creates the program configuration PDA and sets:

- manage authority
- feature flags for adjustable payments, custom deposits, and dispute deterrent

The manage authority can later update those feature flags for new listings and
can pause or resume new exposure. Existing listing and order terms remain
snapshotted in their accounts where the setting can affect settlement.

### 2. Create A Listing

`create_listing_token` lets a creator publish reusable Structured Promise terms.
The creator chooses whether they are the payer or payee side.

The program validates:

- listing ID format
- nonzero capacity
- optional counterparty restrictions
- feature flags for adjustable payment and custom deposits
- expiration timestamp
- required encryption public key metadata

The program then creates the `Listing` PDA and its listing token vault. This is
the first MEE step: the creator locks the creator side's required token amount
into the listing vault.

- payer listing: payment amount plus payer deposit, multiplied by capacity
- payee listing: payee deposit, multiplied by capacity

The creator also pre-funds one future `PartyOrder` rent reserve for each unused
capacity slot. This reserve remains the creator's SOL in the program-owned
Listing account until acceptance or explicit capacity/listing cleanup.

### 3. Accept A Listing

`accept_listing_token` creates one active Structured Promise represented by two
mirrored `PartyOrder` PDAs.

The program validates:

- listing capacity is available
- listing has not expired
- counterparty matches if the listing is restricted
- mint and listing vault match the listing state
- listing vault has enough funds for one accepted order

The program creates:

- a Buyer `PartyOrder` PDA
- a Seller `PartyOrder` PDA
- an accountless order-authority PDA
- an order token vault PDA

The online accepter fronts both state-account initializations. In the same
atomic instruction, the listing reimburses exactly the creator-side copy rent
from the creator-funded reserve. The accepter's final SOL cost is therefore
their own copy plus the shared token vault. The accepter is recorded as the
vault-rent refund recipient.

It transfers the creator side's locked amount from the listing vault into the
order vault, then transfers the counterparty side's required amount from the
counterparty token account into the order vault. At this point MEE is active:
both sides have value exposed to the settlement rules. The order starts active
and ready for confirmation, cancellation, proposals, or messages.

### 4. Confirm An Order

`payer_confirm_token_order` is the payer confirmation action. It settles an
active order according to the locked payment and deposit terms.

On confirmation, the program:

- calculates protocol fee shares
- transfers payer refund/deposit remainder to the payer
- transfers payment and payee refund/deposit remainder to the payee
- transfers protocol fees to the deterministic per-mint protocol fee vault
- writes the same terminal state to both retained order copies
- decreases the parent listing's active order count
- emits an `OrderConfirmed` event

The base confirmation fee is calculated from the payment amount. Additional
proposal deterrent fees may apply if enabled and proposal actions occurred.

### 5. Cancel An Order

`payee_cancel_token_order` handles the payee cancellation/refusal path.

On cancellation, the program calculates cancellation fees, routes fees to the
deterministic per-mint protocol fee vault, refunds the remaining locked amounts,
writes terminal state to both copies, and updates the parent listing counters.
It does not automatically close the order vault or either participant copy.

### 6. Explicit Cleanup

Settlement never reclaims rent automatically:

- `close_order_vault` requires both matching terminal copies and an empty order
  token vault. Either participant may invoke it, but vault rent always returns
  to the recorded accepter who funded it. Both copies are updated with
  `vault_closed = true`.
- `close_my_order_copy` requires only one terminal, vault-closed copy and its
  stored authority signer. It closes only that participant's copy and returns
  only that copy's SOL rent to that participant.
- `deactivate_listing`, `close_listing_vault`, and `close_listing` are explicit
  creator actions. Listing cleanup requires zero active orders; closing the
  Listing returns unused creator-copy reserve and listing rent to the creator.

The protocol, management authority, and counterparty cannot close a user's
retained order copy.

### 7. Proposal Flow

If adjustable payments were enabled for the order, either side can propose a
different payment amount instead of forcing a binary confirm/cancel outcome:

- `payer_make_proposal_order`
- `payee_make_proposal_order`

The opposite side can accept the latest proposal:

- `payer_accept_proposal_token_order`
- `payee_accept_proposal_token_order`

The order `version` protects against accepting stale proposal state. Proposal
expiry can be set. When dispute deterrent is enabled, proposal actions can add
basis-point fees to the relevant side, capped by program constants. This is the
program's current eDDE dispute-convergence path: participant proposals become
recognized state changes with settlement consequences. The order uses the
deterrent setting snapshotted at listing creation, not the current Config value.

### 8. Messages

Participants can attach or update short message metadata:

- `set_payer_order_message`
- `set_payee_order_message`

Messages can be plaintext or encrypted off-chain. The program stores only the
message string, encryption flag, ephemeral public key, nonce, and timestamp. It
does not perform message encryption or decryption.

### 9. Direct Pay

`direct_pay_token` supports a direct token payment path. It transfers a token
amount from payer to payee and routes any configured fee behavior through the
program's deterministic per-mint fee vault.

### 10. Withdraw Protocol Fees

`withdraw_protocol_fees` allows the current manage authority to transfer an
amount accumulated in a per-mint protocol fee vault to a token account for the
same mint. This changes treasury custody only after fees have accrued. It does
not change any listing, order, fee formula, or future settlement destination.

## Events

The program emits Anchor events for off-chain consumers:

- `ListingCreated`
- `ListingDeactivated`
- `ListingVaultClosed`
- `ListingClosed`
- `ListingCapacityAdjusted`
- `OrderCreated`
- `OrderClosed`
- `OrderCancelled`
- `OrderConfirmed`
- `OrderProposal`
- `OrderProposalResponded`
- `PayerOrderMessageSet`
- `PayeeOrderMessageSet`
- `DirectTokenPaid`
- `ConfigInitialized`
- `ConfigUpdated`
- `ProgramPauseUpdated`
- `ProtocolFeesWithdrawn`

Indexers and frontends should use these events together with on-chain account
state to build user-facing order and listing views.

## What The Program Does Not Do

- It does not custody private keys.
- It does not encrypt or decrypt messages on-chain.
- It does not decide whether a real-world obligation was fulfilled.
- It does not automatically observe off-chain silence or external deadlines;
  those must be represented through on-chain fields, participant actions, and
  frontend or indexer interpretation.
- It does not guarantee that public source availability is a security audit.
- It does not grant third parties permission to redeploy or reuse the code; see
  `LICENSE`.

## Verification Scope

This repository is published so users and tooling can compare the public source
with the deployed Solana program binary. See `VERIFY.md` for the verification
workflow.
