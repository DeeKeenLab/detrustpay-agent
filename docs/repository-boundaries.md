# Repository Boundaries

## Canonical repository: `JerryRenCA/TrustPay-v1`

The platform repository owns:

- the canonical Anchor program and protocol tests
- the existing human-facing web application and admin console
- the .NET API, identity modules, indexer, and persistence
- cloud deployment and operational infrastructure
- product books, whitepapers, and long-form material
- the devnet simulation bot
- historical development branches and Git history

Protocol behavior is changed and tested there first. The monorepo is not
mirrored into the Eternal repository because the agent project needs a small,
focused, reproducible surface rather than the full platform and deployment
history.

## Agent repository: `DeeKeenLab/detrustpay-agent`

The agent repository owns:

- a pinned Anchor program and IDL snapshot required to understand integration
- the agent task schema
- the TypeScript SDK
- the MCP reference interface
- requester and provider example agents
- the task lifecycle explorer
- agent-focused lifecycle tests
- Eternal weekly evidence, metrics, demo scripts, and submission material

## Integration boundary

Agent code should integrate with Solana directly through the SDK. A hosted
indexing API may be used as an optional read accelerator, but on-chain accounts
and transactions remain canonical.

The `program/` snapshot is read-only integration input, not a second canonical
implementation. Every update must record the upstream commit, source tree hash,
program ID, and IDL checksum in the same commit.

The agent repository must not depend on uncommitted files from the platform
workspace. Shared code moves only through an explicit, reviewed import.
