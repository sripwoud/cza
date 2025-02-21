import { useAtomValue } from 'jotai'
import { useReadGroth16VerifierVerifyProof } from 'l/wagmi'
import { BarLoader } from 'react-spinners'
import { verifyArgsAtom } from 's/atoms'

export function Verify() {
  const args = useAtomValue(verifyArgsAtom)
  const {
    data: result,
    error,
    isLoading,
    refetch: verify,
  } = useReadGroth16VerifierVerifyProof({
    args: args.inner,
    address: '0x5FbDB2315678afecb367f032d93F642f64180aa3',
    chainId: 31337,
    query: { enabled: false },
  })
  console.log({ args })

  if (args.isNone()) return <div>Proof not available, submit one ☝️</div>
  if (error !== null) return <div>Error: {error.message}</div>
  if (isLoading) return <BarLoader />
  if (result !== undefined)
    return <div>Verification result: {result.toString()}</div>
  return (
    <button onClick={() => verify()} type='button'>
      Verify
    </button>
  )
}
