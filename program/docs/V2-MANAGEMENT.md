# Version 2 Management Model

Version 2 keeps one simple `manage_authority`. It does not introduce a
multisig, timelock, policy registry, or transaction-level administrator.

The management boundary is:

> Management can control admission, pause new exposure, select defaults for new
> listings, and withdraw accrued protocol fees. It cannot change the economic
> rules already snapshotted into a funded Structured Promise.

## Breaking account changes

- `Config.version` is removed.
- `Config.fee_vault_account` is removed.
- all legacy Config parsing and `migrate_config_v2` logic are removed.
- `Listing.dispute_deterrent_enabled` is added.
- `Order.dispute_deterrent_enabled` is added to the shared payload.
- the single Order account is replaced by Buyer and Seller `PartyOrder` copies.
- `PartyOrder.authority` is fixed at raw account-data offset 8.
- listing capacity includes a creator-funded future-copy rent reserve.
- the Config reserved area is recreated for the new layout.

This source must not be deployed as an upgrade over version 1. It requires a
fresh program ID and a newly initialized Config PDA. Listing and Order layouts
are also incompatible with version 1 accounts.

## Snapshotted dispute behavior

`enable_dispute_deterrent` remains a Config default. Listing creation copies
the current value into the new Listing. Acceptance copies that immutable value
into the Order. Proposal instructions read only the Order snapshot.

Changing the Config therefore affects only listings created afterward. It does
not change proposal fees for an existing listing or accepted order.

## Adjustable-payment enforcement

Both payer and payee proposal instructions reject an Order whose
`is_adjustable_payment` value is false. The global feature flag controls whether
new adjustable listings can be created; the Order field controls whether a
proposal is valid for that accepted transaction.

## Backend-independent retained-order discovery

Every accepted order has a Buyer `PartyOrder` copy controlled by the payer and
a Seller copy controlled by the payee. `PartyOrder.authority` is the first
serialized field after the eight-byte Anchor discriminator, so its raw offset is
8 for both roles.

A client calls `getProgramAccounts` once with the `PartyOrder` discriminator and
a wallet memcmp filter at offset 8. Filtering happens at the RPC node; the client
does not download every program account and does not require a DeTrustPay
indexer. `PartyOrder::AUTHORITY_MEMCMP_OFFSET` and serialization tests protect
the layout.

Terminal settlement retains both copies. Each participant may later call
`close_my_order_copy` to reclaim only their own rent, after the shared order
vault has been explicitly closed. Retaining a copy preserves direct chain-state
recovery; closing it intentionally gives up that guarantee.

The management authority has no instruction that closes participant order
copies, the shared order vault, or a Listing.

## Protocol fee vaults

For each mint, the fee vault is derived as:

```text
PDA(["protocol_fee_vault", mint], program_id)
```

The token account is owned by the token program and its token authority is the
Config PDA. Confirmation, cancellation, proposal settlement, and direct pay
all route fees to this stable address. No management update can redirect a
funded order's settlement fee.

`withdraw_protocol_fees` requires the current `manage_authority`. It transfers
an explicit amount to a destination token account for the same mint. Treasury
withdrawal changes custody of fees only after they have accrued; it does not
change any order, fee formula, or settlement path.

## Pause behavior

The existing pause scope is preserved. Pausing blocks:

- listing creation;
- listing acceptance; and
- direct payment.

It does not block participant actions needed to settle an already accepted
order. This prevents management from using the emergency admission control as
transaction-level settlement control.

## Integration impact

Before version 2 can be deployed and used, maintainers must:

1. use the fresh version 2 program ID on each supported client surface;
2. initialize the new Config after the initial deployment;
3. distribute the regenerated version 2 IDL;
4. update clients to derive the per-mint protocol fee vault PDA;
5. remove Config version, migration, and fee-authority UI paths;
6. update proposal and lifecycle builders to pass both `PartyOrder` copies;
7. replace implicit cleanup with explicit vault/copy/listing close builders; and
8. query `PartyOrder` with the discriminator and authority offset 8 rather than
   downloading every program account.

The `detrustpay-agent` network configuration now targets version 2. The existing
platform frontend, admin console, and simulation bot remain version 1 consumers
until their account decoders and transaction builders are explicitly migrated.
