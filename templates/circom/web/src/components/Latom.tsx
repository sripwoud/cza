import { useLatom } from 'h/useLatom'

export const Latom = () => {
  const { latom, reset, set } = useLatom()

  return latom.mapOrElseSync(
    () => (
      <button type='button' onClick={() => set('some value')}>
        Set
      </button>
    ),
    (value) => (
      <div className='flex items-center gap-2'>
        <button type='button' onClick={reset}>
          Reset
        </button>
        <span>{value}</span>
      </div>
    ),
  )
}
