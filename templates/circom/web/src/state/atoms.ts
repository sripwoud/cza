import { None, Option } from '@hazae41/option'
import { atom } from 'jotai'
import { groth16, type Groth16Proof, type PublicSignals } from 'snarkjs'

export const countAtom = atom<Option<bigint>>(new None())
export const latom = atom<Option<string>>(new None())

export const proofAtom = atom<
  Option<{ proof: Groth16Proof; publicSignals: PublicSignals }>
>(new None())

export const verifyArgsAtom = atom(async (get) =>
  Option.wrap(
    (
      await get(proofAtom).map(({ proof, publicSignals }) => groth16.exportSolidityCallData(proof, publicSignals))
    )
      .mapSync((d) => JSON.parse(`[${d}]`))
      .getOrNull(),
  )
)
