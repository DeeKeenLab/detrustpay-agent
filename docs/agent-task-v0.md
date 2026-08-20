# Agent Task Contract v0

Status: frozen for the Colosseum Eternal sprint on August 13, 2026.

This contract is the agent-facing interpretation of the deployed DeTrustPay
`Listing` and `Order` accounts. It does not change the program and must not
claim that the program evaluates off-chain work.

Machine-readable schemas:

- `packages/sdk/schema/task-v0.schema.json`
- `packages/sdk/schema/receipt-v0.schema.json`

## Supported profile

- Solana devnet only.
- Program `3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL`.
- Circle devnet USDC mint `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`.
- Requester creates a payer-side listing.
- One acceptance per listing (`accept_capacity = 1`).
- Provider becomes the payee when it accepts the listing.
- One custom mutual deposit is applied equally to requester and provider.
- Adjustable payment is optional per task.
- Every asset-moving action is signed outside the SDK and MCP server.

Other mints, payee-created listings, reusable listing capacity, encrypted
receipts, and mainnet are outside v0 even when the program can represent them.

## Canonical data

The task manifest contains the complete human- and agent-readable agreement,
including acceptance criteria and artifact requirements. It must be stored at a
content-addressed URI before listing creation.

The listing stores:

- UUID bytes derived from `taskId`;
- the task title;
- a compact task-manifest reference in `description`;
- payment and deposit amounts;
- token mint;
- optional restricted provider;
- the acceptance expiry; and
- whether adjusted settlement is enabled.

The SDK must compare the manifest against on-chain terms. When they disagree,
the on-chain values are canonical for settlement and the SDK must return a
`TASK_TERMS_MISMATCH` error instead of accepting the task.

Amounts are decimal strings in manifests and `bigint` values in TypeScript.
Floating-point token amounts are never accepted by transaction builders.

## Compact references

### Task manifest

The listing `description` uses:

```text
dtp:t0:<content-addressed-uri>
```

### Delivery receipt

The provider stores a receipt manifest that validates against
`receipt-v0.schema.json`, publishes it at a content-addressed URI, and writes
this plaintext value through `set_payee_order_message`:

```text
dtp:r0:<content-addressed-uri>
```

Both encodings are ASCII and must fit their on-chain UTF-8 byte limits. The
delivery message must be no more than 128 bytes. V0 accepts `ipfs://` and
`ar://` references. The artifact may be encrypted off-chain, but the receipt
pointer remains public so an explorer can prove what was submitted.

The message transaction proves which provider address attached the receipt.
The receipt proves artifact integrity through its SHA-256 field. Neither fact
proves that the artifact meets the acceptance criteria.

## Lifecycle mapping

| Agent operation | Program operation | Required signer |
|---|---|---|
| `createTask` | `create_listing_token` | requester/payer |
| `searchTasks` | read open `Listing` accounts | none |
| `getTask` | read `Listing`, retained `PartyOrder`, and signatures | none |
| `acceptTask` | `accept_listing_token` | provider/payee |
| `submitDelivery` | `set_payee_order_message` | provider/payee |
| `confirmDelivery` | `payer_confirm_token_order` | requester/payer |
| `proposeSettlement` | payer or payee proposal instruction | proposing participant |
| `acceptSettlement` | corresponding proposal acceptance | responding participant |
| `cancelTask` | `payee_cancel_token_order` | provider/payee |

Every write method constructs a transaction or sign request. It does not read a
private key, send a transaction, or infer approval from an agent response.

## Deadline semantics

`acceptBy` maps to the on-chain listing expiration and is enforced when a
provider accepts.

`deliverBy` is part of the immutable task manifest and agent policy, but the
current program does not automatically observe or settle an order when that
deadline passes. Participant action is still required. SDK, MCP, examples, and
marketing must describe this distinction explicitly.

## SDK validation

The SDK rejects a write before wallet presentation when:

- cluster, program, or mint differs from the v0 configuration;
- a UUID is invalid or all zeroes;
- a string exceeds its UTF-8 byte limit;
- an amount is negative, fractional, unsafe, or exceeds `u64`;
- payment is zero;
- the mutual deposit is zero, below 25%, or above 400% of payment;
- payer and payee deposit instruction values differ;
- acceptance capacity differs from one;
- requester and restricted provider are the same;
- acceptance expiry has passed;
- the provider is not the restricted counterparty;
- a receipt URI is not content-addressed or exceeds 128 bytes after encoding;
- an order is closed;
- a proposal version is stale or its expiry has passed; or
- decoded accounts, PDAs, mint, vault, or participant roles do not match.

Wallet policy may impose lower spending, deposit, deadline, or counterparty
limits. The SDK may never silently relax those limits.
