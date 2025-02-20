import { useWriteCounterIncrement } from 'l/wagmi'

export const Increment = () => {
  const { writeContract: increment } = useWriteCounterIncrement()
  return (
    // @ts-ignore
    <button type='button' onClick={increment} className='btn btn-primary'>
      Increment
    </button>
  )
}
