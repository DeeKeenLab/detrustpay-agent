# ADR 0001: Eternal v0 Product Contract

- Status: accepted
- Date: August 13, 2026
- Decision owners: DeTrustPay maintainers

## Context

The repository starts Eternal with a deployed conditional-settlement program
but without an agent schema, SDK, MCP interface, examples, or independent
integration. The four-week sprint needs one coherent integration profile rather
than exposing every protocol variation.

## Decision

1. The product is a conditional settlement layer for asynchronous agent work,
   not an agent framework, marketplace, quality oracle, or arbitration system.
2. Eternal supports Solana devnet and the pinned DeTrustPay program only.
3. V0 uses requester-created, payer-side, single-capacity listings denominated
   in Circle devnet USDC. One custom mutual-deposit value is applied equally to
   payer and payee and must remain within the program's 25%-to-400% range.
4. Complete task terms and delivery receipts live in content-addressed storage.
   The chain stores bounded references and settlement state.
5. On-chain terms are canonical for funds. A manifest mismatch is a hard SDK
   error.
6. Listing acceptance expiry is enforced on-chain. Delivery deadlines are
   off-chain policy until the canonical program implements a corresponding
   transition.
7. SDK and MCP write operations return transactions or signing requests. They
   never custody or accept private keys.
8. Direct Solana reads are sufficient for v0 correctness. An indexer and task
   explorer are optional read accelerators and never become canonical.
9. The imported program retains its existing source-availability terms. New
   SDK, MCP, explorer, schemas, and examples should be released under
   Apache-2.0 before their first public package or integration release. The
   applicable per-directory license and SPDX metadata must be added with the
   first implementation commit; this ADR alone is not a license grant.

## Consequences

- The first end-to-end demo is narrow enough to test exhaustively.
- Developers get one mint, role mapping, and signing model instead of a matrix
  of partially tested modes.
- Acceptance criteria remain inspectable but cannot be advertised as verified
  by Solana.
- USDC funding is an onboarding dependency for external devnet testers.
- Payee-created tasks, repeated-capacity listings, mainnet, private receipts,
  automatic timeouts, and autonomous key custody require later decisions.
