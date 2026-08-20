# Contributing

DeTrustPay Agent is devnet-only during the Eternal sprint. Keep changes small,
reviewable, and tied to a reproducible test or devnet transaction.

## Before opening a change

```sh
npm ci
make build
make test
make program-check
make program-test
make json-check
make public-audit
```

Do not commit wallets, keypairs, seed phrases, environment files, RPC
credentials, Terraform state, validator ledgers, or generated build output.
Every asset-moving API must produce an unsigned transaction or explicit signing
request; it must never accept private-key material.

When adding lifecycle behavior, update the corresponding Eternal evidence and
clearly separate team-controlled testing from independent external usage.
