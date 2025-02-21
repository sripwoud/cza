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
    address: '0xb70ff8c130330dd79ce0b525570764680c0e07dc',
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
