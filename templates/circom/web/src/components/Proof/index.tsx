import { Prove } from './Prove'
import { Verify } from './Verify'

export function Proof() {
  return (
    <div className='flex flex-col gap-3 justify-center items-center w-auto max-w-[400px] mx-auto'>
      <Prove />
      <Verify />
    </div>
  )
}
