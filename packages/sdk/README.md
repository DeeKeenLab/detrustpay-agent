# Agent SDK

Planned Eternal deliverable: a TypeScript SDK for constructing and reading the
DeTrustPay agent-task lifecycle.

Target surface:

```text
createTask()
searchTasks()
acceptTask()
submitDelivery()
confirmDelivery()
proposeSettlement()
acceptSettlement()
getTask()
```

The SDK should return transactions or signing requests. It must not require
custody of an agent's private key.
