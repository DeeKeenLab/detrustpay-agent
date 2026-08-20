# Dual Order PDA Implementation Plan

Status: implemented, locally validated, and deployed on devnet

This plan defines the fresh DeTrustPay program layout. It has no migration or
compatibility requirement with the closed devnet deployment.

## 1. Objective

Replace the single shared `Order` account with two complete, synchronized order
copies:

- one buyer order PDA controlled by the buyer;
- one seller order PDA controlled by the seller.

The chain must be sufficient to discover and restore a participant's retained
orders without a DeTrustPay web server or backend index. IndexedDB is a cache of
chain state, not an authority.

## 2. Non-negotiable invariants

1. Solana technically owns every state PDA through the DeTrustPay program. The
   `authority` stored in an order copy is the participant's exclusive protocol
   authority over that copy.
2. The buyer is the only close authority and rent-refund recipient for the buyer
   copy.
3. The seller is the only close authority and rent-refund recipient for the
   seller copy.
4. The buyer copy is economically funded with the buyer's SOL. The seller copy
   is economically funded with the seller's SOL. A counterparty must never bear
   the other participant's final rent cost.
5. No management instruction, settlement instruction, or counterparty may close
   a participant's copy.
6. No protocol instruction automatically closes an order, listing, or token
   vault as a side effect of settlement.
7. A participant may close their copy only after the order is terminal and the
   order token vault has been explicitly closed.
8. Closing one participant's copy must not close or alter the other copy.
9. Before the terminal state, every order mutation receives and updates both
   copies atomically in one Solana transaction.
10. Accepted economic and dispute rules are snapshotted into both copies and
    cannot be changed by later management updates.

## 3. Accounts and PDA seeds

### 3.1 PartyOrder

Use one account type for both roles:

```text
PartyOrder PDA = [
  "party_order",
  listing_pubkey,
  instance_index_le,
  "buyer" | "seller"
]
```

The fixed-width, RPC-queryable header comes first:

```text
authority: Pubkey             // account-data offset 8
counterparty: Pubkey          // offset 40
role: PartyRole               // Buyer or Seller
state_digest: [u8; 32]
bump: u8
order: Order                  // complete shared payload
```

The remaining payload contains a full copy of all order data needed for display,
proposals, messaging, settlement, and independent restoration. This includes the
accepted listing terms, mint, token accounts, deposits, payment amount, snapshotted
feature flags, messages, proposal data, dates, terminal reason, vault address,
vault-rent payer, and whether the vault was explicitly closed.

`authority` is immutable. There is no authority-transfer or delegate instruction.

### 3.2 Order authority and token vault

The signing authority for the order token vault must be an accountless PDA:

```text
OrderAuthority = ["order_authority", parent_listing, instance_index_le]
OrderTokenVault = ["order_token_vault", parent_listing, instance_index_le]
```

Neither participant copy is the token authority. Closing either copy therefore
cannot affect settlement authority.

The listing accepter funds the token-vault rent and is permanently recorded as
`vault_rent_refund_recipient`. Either participant may execute the explicit vault
close after settlement, but its SOL refund always goes to the recorded payer.

### 3.3 Listing

A listing remains a single creator-controlled public account. Its queryable
fixed-width header must precede strings:

```text
creator: Pubkey               // offset 8
counterparty: Pubkey          // zero means open listing
mint: Pubkey
status: ListingStatus
listing_side: ListingSide
category: u8
accept_capacity: u64
used_capacity: u64
active_orders: u64
next_order_index: u64
revision: u64
```

The listing creator is the only listing close authority and the only recipient
of the listing account and listing-vault rent.

## 4. Listing creator rent reserve

Open listing acceptance has only the accepter online. The listing creator must
therefore pre-fund their future order-copy rent when creating or increasing a
listing's capacity.

At listing creation:

```text
creator_copy_reserve =
  minimum_balance(PartyOrder::SPACE) * accept_capacity
```

Store the reserve as excess lamports in the program-owned Listing account and
track the reserved amount explicitly. It is the listing creator's SOL.

During acceptance, Anchor may require the accepter to front both account
initializations. In the same atomic instruction, reimburse exactly the creator
copy's current rent-exempt amount from the creator-funded listing reserve. The
transaction either completes in full or rolls back, so the accepter's final cost
is only their own order copy plus the shared token vault.

Acceptance must fail if the reserve cannot cover the current rent-exempt minimum.
The creator can explicitly top up the listing. Capacity increases add token
collateral and creator-copy reserve atomically. Capacity reductions or listing
closure return only unused reserve to the creator.

After every reserve debit, require:

```text
listing_lamports >= listing_rent_minimum + recorded_unused_reserve
```

## 5. Pair validation and atomic mutation

Add a shared helper used by every non-close order instruction:

```text
validate_order_pair(buyer_copy, seller_copy)
mutate_order_pair(buyer_copy, seller_copy, mutation)
```

Validation must prove:

- correct Buyer and Seller roles;
- reciprocal authorities and counterparties;
- identical listing, instance, mint, token accounts, and vault;
- identical revision, status, and shared state;
- PDA derivations match their immutable identities;
- each stored digest matches a digest recomputed from its shared payload;
- both recomputed shared-state digests are equal.

Apply the transition once, write the resulting shared payload to both copies,
increment both revisions, and recompute both digests. Solana transaction atomicity
prevents a partial update.

Measure compute usage for digest validation. If full serialization hashing is too
expensive, retain an equivalently strong canonical comparison; do not weaken the
pair-consistency invariant merely to save compute units.

## 6. Instruction changes

### 6.1 Listing lifecycle

- `create_listing_token`: use the new query header and collect creator-copy rent
  reserve for all initial capacity.
- `adjust_listing_capacity_token`: adjust token collateral and unused creator-copy
  reserve together; a zero delta may top up a stale reserve without changing
  capacity.
- `accept_listing_token`: initialize Buyer and Seller `PartyOrder` PDAs, reimburse
  the creator-side copy from the listing reserve, initialize the token vault, and
  write identical shared order state.
- `deactivate_listing`: stop future acceptance but retain the account.
- `close_listing_vault`: explicit creator action; require zero active orders and
  return remaining tokens and vault rent to the creator.
- `close_listing`: explicit creator action; require inactive status, zero active
  orders, closed listing vault, and no allocated reserve. Return unused reserve
  and listing-account rent to the creator.

### 6.2 Active order lifecycle

Refactor all of the following to receive, validate, and atomically update both
copies:

- buyer message;
- seller message;
- buyer proposal;
- seller proposal;
- buyer proposal acceptance;
- seller proposal acceptance;
- buyer confirmation;
- seller cancellation.

Settlement transfers tokens and writes terminal state to both copies. It does not
close either copy or the token vault.

Messages remain bounded settlement-related fields, as in the current protocol.
They are not an append-only chat history.

### 6.3 Explicit order cleanup

Add `close_order_vault`:

- requires both copies;
- requires matching terminal state;
- requires a zero token balance;
- may be signed by either participant;
- returns token-account rent only to `vault_rent_refund_recipient`;
- marks `vault_closed = true` in both copies;
- performs no participant-copy close.

Replace `close_token_order` with `close_my_order_copy`:

- requires only the retained copy and its `authority` signer;
- requires terminal status and `vault_closed = true`;
- closes only that copy;
- transfers all of that copy's lamports to its authority;
- cannot be called by management or the counterparty.

Requiring vault closure first guarantees that both copies remain available for
the final shared vault transition. Afterward, the buyer and seller can reclaim
their copy rent independently and in either order.

## 7. Query and local restoration contract

The frontend discovers a wallet's retained order copies with one filtered
`getProgramAccounts` request:

```text
account discriminator = PartyOrder
memcmp offset 8 = connected wallet
```

Use a sliced header query first. Compare `revision`, status, and `state_digest`
with IndexedDB, then fetch full data only for new or changed accounts. Subscribe
to matching program-account changes while online and periodically reconcile to
recover missed WebSocket notifications.

Listings use separate creator, restricted-counterparty, and open-market filters.
Open marketplace discovery must use fixed status/mint/side/category fields before
fetching variable-length listing content.

Closing a copy intentionally removes its guaranteed current-chain recovery path.
The UI must warn the participant and offer an export before submitting the close.
If both copies are closed, the order cannot be reconstructed from current account
state without relying on archival transaction history.

## 8. Code work sequence

1. Reserve a fresh program ID and update `declare_id!`, `Anchor.toml`, deployment
   configuration, IDL metadata, and client configuration together. Never reuse the
   closed program ID.
2. Wrap the complete shared `Order` payload in `PartyOrder` and add `PartyRole`
   plus a canonical shared-state digest.
3. Reorder `Listing`, add status/revision/reserve accounting, and freeze all RPC
   offsets with serialization tests.
4. Add new seeds, errors, and events for pair mismatch, reserve shortage, explicit
   vault closure, and participant-copy closure.
5. Implement pair validation, canonical digest calculation, and paired mutation
   helpers.
6. Refactor listing creation, capacity adjustment, and acceptance, including exact
   participant balance-delta checks.
7. Refactor every active-order instruction to the pair model.
8. Remove all settlement-time `close = ...` constraints and token `close_account`
   calls.
9. Implement explicit listing, vault, and participant-copy close instructions.
10. Regenerate the IDL and update TypeScript transaction builders and decoders.
11. Replace backend notification/snapshot synchronization with wallet-filtered RPC,
    WebSocket reconciliation, and IndexedDB caching.
12. Mirror the final program, IDL, SDK, and protocol documentation into the
    `detrustpay-agent` repository after the canonical implementation passes.

Steps 1-10 and 12 are complete for the program and `detrustpay-agent` snapshot.
The backend-free platform frontend/cache work in step 11 remains separate
client implementation work.

## 9. Required tests and acceptance gates

### State and query tests

- fixed memcmp offsets for order authority and listing creator/counterparty;
- stable serialization size and discriminator filters;
- buyer and seller payloads/digests match after every transition;
- wrong role, authority, listing, instance, revision, or digest is rejected.

### Rent tests

- creator and accepter final SOL deltas prove each funds only their own copy;
- creator reserve decreases by exactly the creator copy's rent minimum;
- insufficient or stale reserve blocks acceptance;
- capacity changes adjust reserve correctly;
- copy close refunds only its participant;
- vault close refunds only its recorded payer;
- no settlement instruction changes account existence or returns rent.

### Lifecycle tests

- both listing orientations: creator is Buyer and creator is Seller;
- open and restricted listings;
- confirmation, cancellation, both proposal paths, messages, and expiries;
- management feature changes do not affect accepted orders;
- one participant closes while the other retains a restorable copy;
- unauthorized and pre-terminal closes fail;
- listing cannot close while active orders remain;
- local DB rebuild from empty storage finds every retained copy without fetching
  all program accounts.

### Deployment gates

- `cargo fmt --check`;
- Rust unit tests;
- Anchor build with a fresh ID;
- full local-validator lifecycle suite;
- compute-unit and transaction-size checks for the largest settlement path;
- devnet deploy and two-wallet end-to-end test;
- verify the deployed executable before updating the public frontend.

## 10. Local validation record

On 2026-08-13, the implementation passed:

- 16 Rust unit tests, including fixed RPC offsets and pair/digest rejection;
- a clean Anchor SBF build without unsafe stack-frame diagnostics; and
- a real local-validator lifecycle covering Config initialization, listing
  creation, dual-copy acceptance, discriminator-scoped authority queries,
  identical state digests, payer confirmation, explicit order-vault closure,
  independent buyer/seller copy closure and rent recovery, explicit listing
  vault closure, and listing closure.

The local-validator test also caught and fixed a runtime-only issue: creator-side
rent reimbursement initially changed lamports before token CPIs. The final
implementation performs all CPIs first and the balanced direct lamport movement
last, as required by the Solana runtime.

## 11. Version 2 devnet deployment record

On 2026-08-13, the tested artifact was deployed as a new program:

- Program: `3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL`
- Config: `CYrLYtpgk5UNuq3C7pjrYd6XuetTujL3Pu8JSk5ozSd8`
- Deployment slot: `483645189`
- Deployment signature:
  `2zU61wsuJQnVgHnt8QZUgcoX4K5GXmXc69nVSpFPM1MgvFPjYD6oUiHhAcaDKGKp22G25tF2uicDyovfJYgQhU9V`
- Local and deployed executable hash:
  `85d76f2458feaeed38cd4e8c89bbf5b51f6f090c5db926b89b4602479937a24b`

After deployment, the same two-wallet lifecycle test passed against devnet and
explicitly cleaned up both order copies, both token vaults, and the Listing.

## 12. Closed version 1 devnet deployment record

On 2026-08-13, devnet program
`3QE69X6FsKxaop5osBW1WL11cCTMMvDuXJLZ9s2TcCNy` was permanently closed by its
upgrade authority. The close reclaimed `5.43974808 SOL` from ProgramData.

After closure, devnet still reported 23 accounts owned by the old program with a
combined `0.1257672 SOL` in account rent. They are inert and cannot be closed by
the now-absent executable. Any token-program vaults whose authority depended on
the old program must likewise be treated as inaccessible devnet test state. The
fresh program performs no migration from these accounts.
