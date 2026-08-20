# Week 1 Video Script

Status: recorded and submitted August 17, 2026. Final length: 47 seconds.

## Spoken script

Hi, I’m Jerry, building DeTrustPay, a trust-minimized payment and escrow
protocol on Solana.

This is my Colosseum Eternal Week 1 update.

This week, I focused on moving DeTrustPay’s order state fully on-chain.
Previously, the frontend relied on a backend and database to synchronize orders
and restore local state.

I redesigned this around two synchronized PartyOrder accounts: one for the
buyer and one for the seller, with each participant controlling their own copy.

The new Anchor program is now deployed on Solana devnet. I also completed a
two-wallet lifecycle test covering order acceptance, settlement, and explicit
account cleanup.

So my Week 1 milestone is a fully on-chain order-state architecture, deployed
and tested on devnet.

Next week, I’ll migrate the frontend to synchronize directly from Solana and
remove the remaining backend dependency.

## Submitted video

[Watch the 47-second Week 1 update on YouTube](https://www.youtube.com/watch?v=9L8WxJ_noGA).

## Submitted title and description

Title:

```text
DeTrustpay Eternal week1
```

Description:

```text
DeTrustPay is a trust-minimized payment and escrow protocol on Solana, based on
a simple idea: promise off-chain, enforce on-chain. For Colosseum Eternal Week
1, I redesigned DeTrustPay's order architecture to move order state fully
on-chain. Each order now uses synchronized buyer and seller PartyOrder accounts.
```
