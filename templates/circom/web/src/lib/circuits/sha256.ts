import type { readGroth16VerifierVerifyProof } from "l/wagmi";
import { createHash } from "node:crypto";
import type { Groth16Proof, PublicSignals } from "snarkjs";

export const sha256ProofSignals = (preimage: string) => {
  const hash = createHash("sha256");
  hash.update(preimage);
  return { in: Buffer.from(hash.digest("hex"), "hex").toJSON().data };
};

type _BuildArray<
  T,
  N extends number,
  R extends T[] = [],
> = R["length"] extends N ? R : _BuildArray<T, N, [T, ...R]>;

type FixedLengthArray<T, N extends number> = N extends N
  ? number extends N
    ? T[] // If N is too generic, fallback to normal array
    : _BuildArray<T, N>
  : never;

type BigInt32 = FixedLengthArray<bigint, 32>;
type BigInt2 = FixedLengthArray<bigint, 2>;
type BigInt2x2 = FixedLengthArray<BigInt2, 2>;

export const sha256ProofVerifyCalldata = ({
  proof: { pi_a, pi_b, pi_c },
  publicSignals,
}: {
  proof: Groth16Proof;
  publicSignals: PublicSignals;
}): Parameters<typeof readGroth16VerifierVerifyProof>[1]["args"] => {
  if (
    pi_a.length !== 2 ||
    pi_b.length !== 2 ||
    pi_b[0].length !== 2 ||
    pi_b[1].length !== 2 ||
    pi_c.length !== 2 ||
    publicSignals.length !== 32
  ) {
    throw new Error("Invalid proof");
  }

  return [
    pi_a.map(BigInt) as BigInt2,
    pi_b.map((r) => r.map(BigInt)) as BigInt2x2,
    pi_c.map(BigInt) as BigInt2,
    publicSignals.map(BigInt) as BigInt32,
  ];
};
