# DeTrustPay Agent

DeTrustPay Agent is the agent-facing development repository for DeTrustPay, a
conditional settlement protocol on Solana.

AI agents can already pay for immediate API calls. This project focuses on work
that completes later: a requester and provider commit task terms, lock payment
and mutual deposits, attach a delivery receipt, and settle through confirmation
or an agreed adjustment.

## Current status

This repository begins as a pre-Eternal baseline on August 13, 2026.

- Network: Solana devnet only for the Eternal sprint.
- Program: existing DeTrustPay Anchor program snapshot.
- External usage: no meaningful real-user usage yet.
- Agent interface: not implemented at baseline.
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
on-chain settlement consequences.

## Repository map

```text
program/                  Anchor program snapshot and deployed IDL
packages/sdk/             Agent-oriented TypeScript SDK
apps/mcp-server/          MCP reference interface
apps/task-explorer/       Read-only lifecycle explorer
examples/requester-agent/ Reference requester agent
examples/worker-agent/    Reference provider agent
tests/                    Agent lifecycle integration tests
docs/eternal/             Baseline, weekly evidence, metrics, and submission docs
```

Only the `program/` baseline is implemented at repository creation. The agent
components deliberately start as documented work areas so Eternal progress is
visible in Git history.

## Program checks

Requirements include the Rust, Solana, and Anchor-compatible toolchains pinned
by the program lockfile.

```sh
make program-check
make program-build
```

The deployed devnet program ID is:

```text
3QE69X6FsKxaop5osBW1WL11cCTMMvDuXJLZ9s2TcCNy
```

## Repository provenance

The legacy product monorepo remains at
`JerryRenCA/TrustPay`. This repository is a clean, competition-focused source
baseline intended for `DeeKeenLab/detrustpay-agent`; it is not a mirror of the
legacy Git history.

See [repository boundaries](docs/repository-boundaries.md) for what belongs in
each repository.

## License

The imported program retains its existing source-availability license. See
[LICENSE](LICENSE). Licensing for new SDK and integration components should be
decided explicitly before public release.
