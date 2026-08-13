# Security

This repository is devnet-only during the Eternal sprint. Do not use real-value
assets or production keys.

## Boundaries

- The protocol constrains settlement state transitions; it does not verify the
  quality or truth of off-chain work.
- The reference agents and MCP server must never accept, store, log, or transmit
  seed phrases or private keys.
- SDK operations should construct transactions for an external signer unless a
  test-only signer is explicitly configured.
- Delivery artifacts remain off-chain. Only bounded metadata, artifact
  references, and integrity hashes should enter the lifecycle.
- No component should describe the program as audited unless a published audit
  report exists.

## Repository hygiene

Never commit environment files, keypairs, wallet directories, Terraform
variables, build artifacts, validator ledgers, or generated static exports. The
root `.gitignore` covers these classes, but contributors remain responsible for
reviewing every staged change.

Report security concerns privately to the project maintainers. Do not include
private keys, seed phrases, or unredacted credentials in a report.
