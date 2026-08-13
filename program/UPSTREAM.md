# Program Provenance

The contents of this directory are a pinned integration snapshot. The
`JerryRenCA/TrustPay` platform repository remains the canonical source for
protocol development and tests.

## Current pin

- Upstream repository: `https://github.com/JerryRenCA/TrustPay`
- Upstream path: `anchor-detrustpay/`
- Upstream snapshot commit: `f975216561ef29d388986bab81d685f689a3c854`
- Upstream snapshot subject: `repo: make clean-clone builds self-contained`
- Program source commit: `eef2ae7ecde496ecf5090edc6ca56ca39997ea47`
- Program source Git tree: `b088e078300ff2c3bdd902f4453f7cae57df7680`
- Cargo lock Git blob: `5ceb4f4ba479de5c94b34fd3150079a4dda7f449`
- IDL Git blob: `8fd0cdcc6d9617d77369e8dc8830f1fdc9635021`
- Program ID: `3QE69X6FsKxaop5osBW1WL11cCTMMvDuXJLZ9s2TcCNy`
- IDL SHA-256: `9db64a030a3d22cc962328f73669531607382d6748eadbf984e6c515c67e0eab`
- Baseline tag: `pre-eternal-2026-h2`

The program source tree and Cargo lockfile hashes are identical to the pinned
upstream commit. `Anchor.toml` is intentionally reduced for the agent repository
and the deployed IDL is included as integration input.

## Update rule

1. Make and test protocol changes in `JerryRenCA/TrustPay`.
2. Commit or tag the canonical change.
3. Import only the program source, lockfile, required build configuration, and
   deployed IDL into this repository.
4. Update every value in **Current pin** in the same synchronization commit.
5. Run `make program-check` and the agent lifecycle tests before merging.

Do not patch the program snapshot directly during agent feature work.
