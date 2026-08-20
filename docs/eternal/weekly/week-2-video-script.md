# Week 2 Video Script

Status: ready to record after the public repository URL is verified. Target
length: 50–60 seconds.

## Spoken script

Hi, I’m Jerry, building DeTrustPay on Solana.

In Week 1, I moved order state fully on-chain. This week, I removed the backend
from the browser payment flow.

The connected wallet is now the user identity. Listings, synchronized buyer and
seller orders, messages, proposals, and settlement are read from or written
directly to Solana.

The browser keeps a disposable local mirror for speed. Recovery runs only when
the user asks: active accounts use wallet filters, and history loads ten
signatures at a time with a saved cursor.

To protect public RPC endpoints, identical reads share one request, concurrency
is capped at two, and a 429 pauses the whole coordinator with exponential
backoff instead of creating a retry storm.

The devnet app is live, and I published the program, SDK foundation, tests, and
weekly evidence on GitHub.

Next, I’m turning this direct-chain interface into the agent transaction SDK
and externally signed requester and provider examples.

## Recording plan

| Time | Screen | Evidence |
|---|---|---|
| 0:00–0:08 | Week 1 architecture image | Fully on-chain order state |
| 0:08–0:22 | Live wallet and listing/order pages | No backend login or API dependency |
| 0:22–0:38 | Account recovery panel | Explicit sync, ten-signature pages, saved cursor |
| 0:38–0:50 | SDK coordinator source and tests | Deduplication, concurrency two, 429 backoff |
| 0:50–0:58 | Public GitHub README and live site | Reproducible evidence and next step |

## Checklist

- Keep the final edit at or below 60 seconds.
- Show the live devnet site and public repository, not private workspace paths.
- Do not show wallet balances, environment variables, terminal history,
  browser account controls, RPC credentials, or private Colosseum fields.
- Use only team-controlled devnet transactions and label them as tests.
- Upload to Loom or YouTube with link access enabled.
- Add the final URL to `week-2.md`, then submit that URL in Colosseum after the
  Week 2 field opens on August 22.
