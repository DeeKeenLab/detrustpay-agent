# DeTrustPay Agent

DeTrustPay Agent is the agent-facing development repository for DeTrustPay, a
conditional settlement protocol on Solana.

[Live devnet app](https://detrustpay.com) ·
[Week 1 update](https://www.youtube.com/watch?v=9L8WxJ_noGA) ·
[Devnet program](https://explorer.solana.com/address/3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL?cluster=devnet)

AI agents can already pay for immediate API calls. This project focuses on work
that completes later: a requester and provider commit task terms, lock payment
and mutual deposits, attach a delivery receipt, and settle through confirmation
or an agreed adjustment.

## Current status

The tagged pre-Eternal baseline is dated August 13, 2026. Current progress is
recorded as reviewable weekly evidence rather than folded into the baseline.

- Network: Solana devnet only for the Eternal sprint.
- Program: version 2 dual-order snapshot deployed and initialized on devnet.
- Live integration: frozen devnet configuration targets the fresh version 2 ID.
- Week 1: submitted; on-chain PartyOrder architecture and devnet lifecycle.
- Week 2: in progress; backendless wallet recovery and 429-safe RPC reads.
- SDK: policy validation and RPC query coordination implemented and tested;
  transaction construction remains in progress.
- External usage: no meaningful real-user usage yet.
- Security: no published third-party audit; use devnet test assets only.
- Mainnet: explicitly outside the sprint scope.

See [the baseline](docs/eternal/baseline.md) for an exact description of what
existed before the sprint.

## Eternal objective

An external developer should be able to install the SDK and run requester and
provider agents through this lifecycle:

1. Create a task with payment, deposits, deadline, and acceptance criteria.
2. Accept the task and activate mutual economic exposure.
3. Submit a delivery receipt containing an artifact reference and content hash.
4. Confirm delivery or propose an adjusted settlement.
5. Settle deterministically through the DeTrustPay program.

The protocol does not judge off-chain work. It constrains valid actions and
on-chain settlement consequences. Settlement remains controlled by the two
transaction participants: no administrator, oracle, marketplace, or agent
operator can choose the outcome of an individual transaction. See the
[protocol principles](docs/detrustpay-protocol-principles.md) for the complete
trust and responsibility model.

The Eternal v0 contract is now frozen around requester-created, single-capacity
tasks settled in Circle devnet USDC with external signing. See the
[agent task contract](docs/agent-task-v0.md), the
[Day 0 readiness record](docs/eternal/day-0-readiness.md), and the
[acceptance gates](docs/eternal/acceptance-gates.md).

## Repository map

```text
program/                  Canonical v2 source, lifecycle test, and generated IDL
packages/sdk/             Agent-oriented TypeScript SDK
apps/mcp-server/          MCP reference interface
apps/task-explorer/       Read-only lifecycle explorer
examples/requester-agent/ Reference requester agent
examples/worker-agent/    Reference provider agent
tests/                    Agent lifecycle integration tests
docs/eternal/             Baseline, weekly evidence, metrics, and submission docs
config/                   Frozen Eternal network and mint configuration
```

The `program/` directory is a read-only integration snapshot; protocol changes
are made in the platform repository and imported here explicitly. It currently
contains the incompatible version 2 source and IDL. Its network configuration
targets the fresh version 2 devnet deployment; it does not migrate or reuse
version 1 state.

## SDK quickstart

Node.js 20 or newer is required.

```sh
npm ci
npm run build
npm test
```

The current SDK alpha exposes fail-closed Eternal network validation, exact
`bigint` amount policy, UTF-8 byte-limit validation, and a user-driven RPC query
coordinator. The coordinator deduplicates identical reads, limits concurrency
to two by default, caches short-lived results, and backs off globally when a
provider returns HTTP 429. It never starts background polling.

See [the SDK package](packages/sdk/README.md) and
[the Week 2 update](docs/eternal/weekly/week-2.md) for the implemented scope.

## Program checks

Requirements include the Rust, Solana, and Anchor-compatible toolchains pinned
by the program lockfile.

```sh
make program-check
make program-test
make program-build
make program-install
make program-lifecycle-test
```

The version 2 devnet program ID is:

```text
3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL
```

Its Config PDA is `CYrLYtpgk5UNuq3C7pjrYd6XuetTujL3Pu8JSk5ozSd8`.

## Repository provenance

The canonical product and protocol monorepo remains the private
`JerryRenCA/TrustPay-v1` platform repository.
This repository is a clean, competition-focused integration baseline intended
for `DeeKeenLab/detrustpay-agent`; it is not a mirror of the platform history.

See [program provenance](program/UPSTREAM.md) for the pinned source hashes and
[repository boundaries](docs/repository-boundaries.md) for what belongs in each
repository.

## License

The imported program retains its source-availability terms. The SDK is licensed
under Apache-2.0. See the repository [license map](LICENSE) and the
[SDK license](packages/sdk/LICENSE).
