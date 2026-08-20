import assert from "node:assert/strict";
import test from "node:test";

import {
  ETERNAL_DEVNET,
  assertEternalNetwork,
  assertUtf8ByteLength,
  parseBaseUnits,
  validateMutualDeposit,
} from "../src/index.js";

test("base-unit parsing rejects numbers and non-canonical strings", () => {
  assert.equal(parseBaseUnits("9007199254740993"), 9_007_199_254_740_993n);
  assert.throws(() => parseBaseUnits("01"), /unsigned decimal string/);
  assert.throws(() => parseBaseUnits("1.5"), /unsigned decimal string/);
  assert.throws(() => parseBaseUnits(5 as unknown as string), /unsigned decimal string/);
});

test("mutual deposit accepts exactly the 25%-to-400% boundary", () => {
  assert.deepEqual(
    validateMutualDeposit({
      paymentBaseUnits: "100",
      mutualDepositBaseUnits: "25",
    }),
    { payment: 100n, mutualDeposit: 25n },
  );
  assert.deepEqual(
    validateMutualDeposit({
      paymentBaseUnits: "100",
      mutualDepositBaseUnits: "400",
    }),
    { payment: 100n, mutualDeposit: 400n },
  );
  assert.throws(
    () =>
      validateMutualDeposit({
        paymentBaseUnits: "100",
        mutualDepositBaseUnits: "24",
      }),
    /between 25% and 400%/,
  );
  assert.throws(
    () =>
      validateMutualDeposit({
        paymentBaseUnits: "100",
        mutualDepositBaseUnits: "401",
      }),
    /between 25% and 400%/,
  );
});

test("network policy fails closed on a changed program or mint", () => {
  const exact = {
    cluster: ETERNAL_DEVNET.cluster,
    programId: ETERNAL_DEVNET.programId,
    configAccount: ETERNAL_DEVNET.configAccount,
    mint: ETERNAL_DEVNET.mint,
    tokenProgram: ETERNAL_DEVNET.tokenProgram,
  };
  assert.doesNotThrow(() => assertEternalNetwork(exact));
  assert.throws(
    () => assertEternalNetwork({ ...exact, cluster: "mainnet-beta" }),
    /network mismatch for cluster/,
  );
  assert.throws(
    () => assertEternalNetwork({ ...exact, mint: "11111111111111111111111111111111" }),
    /network mismatch for mint/,
  );
});

test("UTF-8 limits count bytes rather than JavaScript characters", () => {
  assert.doesNotThrow(() => assertUtf8ByteLength("ok", 2, "title"));
  assert.doesNotThrow(() => assertUtf8ByteLength("支付", 6, "title"));
  assert.throws(
    () => assertUtf8ByteLength("支付", 5, "title"),
    /6 UTF-8 bytes/,
  );
});
