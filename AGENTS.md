# Repository Guidelines

## Scope

This repository contains only the agent-facing DeTrustPay product, its Solana
program baseline, reference integrations, tests, and Eternal evidence.

Do not import the legacy product's identity system, admin console, cloud
deployment state, generated websites, books, credentials, wallet files, or Git
history.

## Development rules

- Treat Solana devnet as the only supported network during Eternal.
- Keep signing external to the SDK and MCP server by default.
- Add integration coverage for every lifecycle transition.
- Keep simulation activity separate from external-user metrics.
- Record weekly evidence under `docs/eternal/weekly/`.
- Never claim off-chain fulfillment is verified by the program.
- Never commit secrets or test keypairs.

## Commands

- `make program-check`: host-target Rust check.
- `make program-test`: Rust unit tests.
- `make program-build`: Solana SBF build.
- `make repo-status`: concise repository and object-database status.
