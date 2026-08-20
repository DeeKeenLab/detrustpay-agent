# Agent SDK

This package contains the first public, tested building blocks for the
DeTrustPay agent-task lifecycle. The current alpha implements fail-closed
network and amount policy plus a rate-limit-aware RPC query coordinator. It
does not yet construct settlement transactions.

## Implemented alpha surface

```ts
import {
  ETERNAL_DEVNET,
  RpcQueryCoordinator,
  assertEternalNetwork,
  validateMutualDeposit,
} from "@detrustpay/agent-sdk";

assertEternalNetwork({
  cluster: "devnet",
  programId: ETERNAL_DEVNET.programId,
  configAccount: ETERNAL_DEVNET.configAccount,
  mint: ETERNAL_DEVNET.mint,
  tokenProgram: ETERNAL_DEVNET.tokenProgram,
});

const amounts = validateMutualDeposit({
  paymentBaseUnits: "1000000",
  mutualDepositBaseUnits: "500000",
});

const rpc = new RpcQueryCoordinator({ maxConcurrency: 2 });
const orders = await rpc.query(
  `party-orders:${wallet}`,
  () => fetchWalletFilteredPartyOrders(wallet),
);
```

The coordinator never polls by itself. Reads begin only when the caller asks,
identical requests are coalesced, concurrency is bounded, results have a short
TTL, and explicit RPC 429 responses establish a shared exponential-backoff
cooldown.

## Planned transaction surface

Target surface:

```text
createTask()
searchTasks()
acceptTask()
submitDelivery()
confirmDelivery()
proposeSettlement()
acceptSettlement()
getTask()
findMyActiveOrders()
```

The completed SDK will return transactions or signing requests. It must not require
custody of an agent's private key.

The Eternal v0 behavior is frozen in
[`docs/agent-task-v0.md`](../../docs/agent-task-v0.md). Machine-readable task
and receipt schemas live under [`schema/`](schema/). Implementations must use
`bigint` for token base units and validate UTF-8 byte limits before building a
transaction.

The version 2 Order layout supports backend-independent participant discovery.
`findMyActiveOrders()` must issue two filtered `getProgramAccounts` calls for
the Order discriminator, using payer offset `8` and payee offset `40`, then
merge the results. It must not download every Order account and filter locally.
See [`program/docs/V2-MANAGEMENT.md`](../../program/docs/V2-MANAGEMENT.md).
