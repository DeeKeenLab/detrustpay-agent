# Repository Boundaries

## Legacy repository: `JerryRenCA/TrustPay`

The legacy repository remains the historical product monorepo. It owns:

- the existing human-facing web application and admin console
- the .NET API, identity modules, indexer, and persistence
- cloud deployment and operational infrastructure
- product books, whitepapers, and long-form material
- the existing devnet simulation bot
- historical development branches and Git history

It must not be mirrored to the Eternal repository because its working tree and
history contain generated output, obsolete files, local configuration, key
fixtures, infrastructure artifacts, and sensitive development material.

## Agent repository: `DeeKeenLab/detrustpay-agent`

The agent repository owns:

- the clean Anchor program and IDL baseline required to understand integration
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

The new repository must not depend on uncommitted files from the legacy
workspace. Shared code moves only through an explicit, reviewed import.
