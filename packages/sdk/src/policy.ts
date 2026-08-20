export const ETERNAL_DEVNET = Object.freeze({
  cluster: "devnet",
  programId: "3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL",
  configAccount: "CYrLYtpgk5UNuq3C7pjrYd6XuetTujL3Pu8JSk5ozSd8",
  mint: "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU",
  tokenProgram: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
  mintDecimals: 6,
});

export type EternalNetworkInput = {
  cluster: string;
  programId: string;
  configAccount: string;
  mint: string;
  tokenProgram: string;
};

export function assertEternalNetwork(input: EternalNetworkInput): void {
  const expected: Record<keyof EternalNetworkInput, string> = {
    cluster: ETERNAL_DEVNET.cluster,
    programId: ETERNAL_DEVNET.programId,
    configAccount: ETERNAL_DEVNET.configAccount,
    mint: ETERNAL_DEVNET.mint,
    tokenProgram: ETERNAL_DEVNET.tokenProgram,
  };

  for (const key of Object.keys(expected) as Array<keyof EternalNetworkInput>) {
    if (input[key] !== expected[key]) {
      throw new Error(
        `DeTrustPay Eternal network mismatch for ${key}: expected ${expected[key]}.`,
      );
    }
  }
}

export function parseBaseUnits(value: string, fieldName = "amount"): bigint {
  if (typeof value !== "string" || !/^(0|[1-9][0-9]*)$/.test(value)) {
    throw new TypeError(`${fieldName} must be an unsigned decimal string.`);
  }
  return BigInt(value);
}

export function validateMutualDeposit(input: {
  paymentBaseUnits: string;
  mutualDepositBaseUnits: string;
}): { payment: bigint; mutualDeposit: bigint } {
  const payment = parseBaseUnits(input.paymentBaseUnits, "paymentBaseUnits");
  const mutualDeposit = parseBaseUnits(
    input.mutualDepositBaseUnits,
    "mutualDepositBaseUnits",
  );

  if (payment === 0n) {
    throw new RangeError("paymentBaseUnits must be greater than zero.");
  }
  if (mutualDeposit * 4n < payment || mutualDeposit > payment * 4n) {
    throw new RangeError(
      "mutualDepositBaseUnits must be between 25% and 400% of paymentBaseUnits.",
    );
  }
  return { payment, mutualDeposit };
}

export function assertUtf8ByteLength(
  value: string,
  maximumBytes: number,
  fieldName: string,
): void {
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0) {
    throw new RangeError("maximumBytes must be a non-negative safe integer.");
  }
  const bytes = new TextEncoder().encode(value).byteLength;
  if (bytes > maximumBytes) {
    throw new RangeError(
      `${fieldName} is ${bytes} UTF-8 bytes; the maximum is ${maximumBytes}.`,
    );
  }
}
