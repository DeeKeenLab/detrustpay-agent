# DeTrustPay Agent: Product One-Pager

## One sentence

DeTrustPay is the conditional settlement layer for AI-agent work on Solana.

## Problem

Agents can pay instantly for APIs, but many useful tasks finish later and
produce deliverables that cannot be objectively verified at payment time.
Requester agents risk paying for poor or missing work. Provider agents risk
doing work and being denied payment.

## Product

A requester and provider:

1. Commit task terms, payment, deadline, and acceptance criteria.
2. Lock payment and mutual commitment deposits on Solana.
3. Attach a delivery receipt and artifact integrity hash.
4. Confirm successful delivery or negotiate an adjusted payment.
5. Settle through deterministic program rules.

DeTrustPay does not decide whether the work is good. It makes valid actions,
economic exposure, and settlement consequences predictable.

## Initial user

Developers building AI-agent marketplaces, autonomous service agents, and
human-in-the-loop agent workflows.

## Initial use case

Small digital tasks such as research, data transformation, testing, content
production, and code review where a requester can inspect the result.

## Why Solana

Agent settlement needs low fees, fast confirmation, stablecoin support, and
programmable account state. Solana makes multi-step conditional settlement
economically practical for modest task values.

## Four-week outcome

An external developer can install the SDK and run two agents through task
creation, acceptance, delivery, confirmation or adjustment, and devnet
settlement without help from the DeTrustPay team.

## Business direction

- protocol settlement fees
- hosted indexing and monitoring
- developer API and SDK services
- integrations for agent platforms

Deposits are enforcement collateral, not protocol revenue.
