import { None, type Option } from '@hazae41/option'
import { atom } from 'jotai'
import type { Groth16Proof, PublicSignals } from 'snarkjs'

export const countAtom = atom<Option<bigint>>(new None())
export const latom = atom<Option<string>>(new None())

export const proofAtom = atom<
  Option<{ proof: Groth16Proof; publicSignals: PublicSignals }>
>(new None())
