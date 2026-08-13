# Integration Tests

Agent-focused tests will cover:

- task/listing creation
- acceptance and both-side funding
- delivery receipt metadata
- successful confirmation
- permitted cancellation
- proposal creation and adjusted settlement
- stale proposal and expiry rejection
- closed-order rejection
- signer, role, mint, vault, and amount validation

No test keypair may be committed. Test wallets must be generated at runtime or
provided through ignored local configuration.
