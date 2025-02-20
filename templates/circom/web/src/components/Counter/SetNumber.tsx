import { useWriteCounterSetNumber } from 'l/wagmi'
import { useState } from 'react'

export const SetNumber = () => {
  const [number, setNumber] = useState('')

  const { writeContract, isPending } = useWriteCounterSetNumber({
    mutation: {
      onSettled: () => {
        setNumber('')
      },
    },
  })

  return (
    <div className='flex flex-col'>
      <label htmlFor='number'>Set Number</label>
      <div className='flex flex-row'>
        <input
          className='set-number-input'
          disabled={isPending}
          id='number'
          min={0}
          step={1}
          onChange={(e) => setNumber(e.target.value)}
          value={number}
          type='number'
        />
        <button
          className='btn btn-primary'
          disabled={isPending}
          onClick={() => writeContract({ args: [BigInt(number)] })}
          type='button'
        >
          Set
        </button>
      </div>
    </div>
  )
}
