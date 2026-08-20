import assert from "node:assert/strict";
import test from "node:test";

import { RpcQueryCoordinator, isRpcRateLimitError } from "../src/index.js";

const deferred = <T>() => {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => {
    resolve = done;
  });
  return { promise, resolve };
};

test("identical in-flight reads share one RPC operation", async () => {
  const coordinator = new RpcQueryCoordinator();
  const result = deferred<number>();
  let calls = 0;
  const operation = async () => {
    calls += 1;
    return result.promise;
  };

  const first = coordinator.query("wallet:abc", operation);
  const second = coordinator.query("wallet:abc", operation);
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(calls, 1);
  result.resolve(42);
  assert.deepEqual(await Promise.all([first, second]), [42, 42]);
});

test("cache can be bypassed or invalidated by prefix", async () => {
  let now = 1_000;
  const coordinator = new RpcQueryCoordinator({ now: () => now });
  let calls = 0;
  const operation = async () => ++calls;

  assert.equal(await coordinator.query("order:1", operation), 1);
  assert.equal(await coordinator.query("order:1", operation), 1);
  assert.equal(await coordinator.query("order:1", operation, { force: true }), 2);
  coordinator.invalidate("order:");
  assert.equal(await coordinator.query("order:1", operation), 3);
  now += 31_000;
  assert.equal(await coordinator.query("order:1", operation), 4);
});

test("concurrency remains bounded across distinct keys", async () => {
  const coordinator = new RpcQueryCoordinator({ maxConcurrency: 2 });
  const gates = [deferred<number>(), deferred<number>(), deferred<number>()];
  let active = 0;
  let maximum = 0;
  let started = 0;

  const reads = gates.map((gate, index) =>
    coordinator.query(`key:${index}`, async () => {
      started += 1;
      active += 1;
      maximum = Math.max(maximum, active);
      const value = await gate.promise;
      active -= 1;
      return value;
    }),
  );

  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(started, 2);
  gates[0]?.resolve(0);
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(started, 3);
  gates[1]?.resolve(1);
  gates[2]?.resolve(2);
  assert.deepEqual(await Promise.all(reads), [0, 1, 2]);
  assert.equal(maximum, 2);
});

test("429 responses retry with a coordinator-wide cooldown", async () => {
  let now = 5_000;
  const sleeps: number[] = [];
  const coordinator = new RpcQueryCoordinator({
    baseRetryDelayMs: 100,
    now: () => now,
    random: () => 0,
    sleep: async (milliseconds) => {
      sleeps.push(milliseconds);
      now += milliseconds;
    },
  });
  let attempts = 0;

  const value = await coordinator.query("rate-limited", async () => {
    attempts += 1;
    if (attempts === 1) {
      throw new Error("HTTP 429: Too Many Requests");
    }
    return "ok";
  });

  assert.equal(value, "ok");
  assert.equal(attempts, 2);
  assert.deepEqual(sleeps, [100]);
  assert.equal(isRpcRateLimitError(new Error("provider rate limit")), true);
  assert.equal(isRpcRateLimitError(new Error("account missing")), false);
});
