# Program Provenance

The contents of this directory are an integration snapshot. The
`JerryRenCA/TrustPay-v1` platform repository remains the canonical source for
protocol development and tests.

## Current import

- Import status: version 2 dual-order deployment snapshot
- Imported on: 2026-08-13
- Upstream repository: `https://github.com/JerryRenCA/TrustPay-v1`
- Upstream path: `anchor-detrustpay/`
- Canonical base commit: `f975216561ef29d388986bab81d685f689a3c854`
- Canonical base subject: `repo: make clean-clone builds self-contained`
- Canonical source state: uncommitted version 2 working-tree changes
- Program crate version: `2.0.0`
- Program source-tree SHA-256: `c9d801669e4de37355b07e48ad9c656723e26c12a3b6c0d01f98751ce08fbb69`
- Cargo lockfile SHA-256: `fbafcc7a1cbd19dd8f9c863357a3334c38e6132bf34285dd72e16931a651e40f`
- Program manifest SHA-256: `0c0561b0e64cd34f80706f7cac67ecc108b0fb6d251d8e4fad72f5d851aa8a3a`
- IDL SHA-256: `f7d61d65153a0503b5fc4998bd435fecad1144bd25991cee4eb0b736af50479f`
- Version 2 program ID: `3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL`
- Config PDA: `CYrLYtpgk5UNuq3C7pjrYd6XuetTujL3Pu8JSk5ozSd8`
- Deployment slot: `483645189`
- Deployment signature: `2zU61wsuJQnVgHnt8QZUgcoX4K5GXmXc69nVSpFPM1MgvFPjYD6oUiHhAcaDKGKp22G25tF2uicDyovfJYgQhU9V`
- Deployed executable hash: `85d76f2458feaeed38cd4e8c89bbf5b51f6f090c5db926b89b4602479937a24b`

The source tree, manifest, lockfile, candidate IDL, program overview, and
version 2 management notes are byte-identical to the canonical working tree at
the time of import. `Anchor.toml` remains agent-repository-specific and supports
devnet plus local lifecycle testing.

The version 1 devnet program was permanently closed. Version 2 uses the fresh
address above and performs no migration from old accounts.

## Version 2 import scope

This import includes:

- immutable dispute-deterrent policy snapshots on Listing and Order accounts;
- deterministic program-controlled protocol fee vaults per mint;
- manage-authority withdrawal of already accrued protocol fees;
- proposal rejection for orders without adjustable payment enabled;
- backend-independent retained-order discovery through the fixed authority RPC
  filter offset;
- mirrored Buyer and Seller `PartyOrder` accounts with authority at offset 8;
- creator-funded future-copy rent reserve on listings;
- explicit, participant-owned order-copy and vault cleanup; and
- removal of Config versioning, migration logic, and replaceable fee-vault
  configuration.

See [the management model](docs/V2-MANAGEMENT.md) for the complete breaking
change and integration impact.

## Update rule

1. Make and test protocol changes in `JerryRenCA/TrustPay-v1`.
2. Commit or tag the canonical change.
3. Import only the program source, lockfile, required build configuration,
   documentation, and generated IDL into this repository.
4. Update every value in **Current import** in the same synchronization commit.
5. Run `make program-check`, `make program-test`, and the agent lifecycle tests
   before merging.

This candidate import must be repinned to the eventual canonical version 2
commit before it is merged or released. Do not patch the program snapshot
directly during agent feature work.
