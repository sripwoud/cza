import { Option } from '@hazae41/option'
import { useAtom } from 'jotai'
import { useCallback } from 'react'
import { latom } from 's/atoms'

export const useLatom = () => {
  const [atom, setAtom] = useAtom(latom)

  const set = useCallback(
    (value?: string) => {
      setAtom(Option.wrap(value))
    },
    [setAtom],
  )

  const reset = useCallback(() => {
    set()
  }, [set])

  return { latom: atom, reset, set }
}
