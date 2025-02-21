import { Option } from '@hazae41/option'
import { useForm } from '@tanstack/react-form'
import { FieldInfo } from 'c/FieldInfo'
import { useAtom } from 'jotai'
import { sha256ProofSignals } from 'l/circuits/sha256'
import { capitalize } from 'l/format'
import { type PreimageSchema, preimageSchema } from 'l/schemas'
import type { FormEvent } from 'react'
import { proofAtom } from 's/atoms'
import { groth16 } from 'snarkjs'

export function Prove() {
  const [proof, setProof] = useAtom(proofAtom)
  const form = useForm<PreimageSchema>({
    defaultValues: { preimage: '' },
    onSubmit: async ({ value: { preimage } }) => {
      const proof = await groth16.fullProve(
        sha256ProofSignals(preimage),
        '/sha256_32.wasm',
        '/sha256_32.zkey',
      )
      console.log(proof)
      setProof(Option.wrap(proof))
    },
    validators: { onChange: preimageSchema },
  })

  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault()
    e.stopPropagation()
    form.handleSubmit()
  }

  return proof.mapOrSync(
    <form className='flex flex-col gap-4' onSubmit={(e) => handleSubmit(e)}>
      <form.Field
        name='preimage'
        children={(field) => (
          <>
            <label htmlFor={field.name}>{capitalize(field.name)}</label>
            <input
              id={field.name}
              name={field.name}
              onChange={(e) => field.handleChange(e.target.value)}
              placeholder='hello zk circom app template'
              type='text'
              value={field.state.value}
            />
            <FieldInfo field={field} />
          </>
        )}
      />
      <form.Subscribe
        selector={({ canSubmit, isSubmitting }) => [canSubmit, isSubmitting]}
        children={([canSubmit, isSubmitting]) => (
          <button type='submit' aria-busy={isSubmitting} disabled={!canSubmit}>
            Submit
          </button>
        )}
      />
    </form>,
    () => null,
  )
}
