import { useAtomValue } from 'jotai'
import { sha256ProofVerifyCalldata } from 'l/circuits/sha256'
import { useReadGroth16VerifierVerifyProof } from 'l/wagmi'
import { BarLoader } from 'react-spinners'
import { proofAtom } from 's/atoms'

export function Verify() {
  const proof = useAtomValue(proofAtom)

  return proof.mapOrSync(
    <div>Proof not available, submit one ☝️</div>,
    (proof) => {
      const {
        data,
        isError,
        isLoading,
        refetch: _verify,
      } = useReadGroth16VerifierVerifyProof({
        args: sha256ProofVerifyCalldata(proof),
        query: { enabled: false },
      })

      if (isError) return <div>Error</div>
      if (isLoading) return <BarLoader />
      if (data !== undefined)
        return <div>Verification result: {data.toString()}</div>
      return (
        <button onClick={() => console.log('verify')} type='button'>
          Verify
        </button>
      )
    },
  )
}
