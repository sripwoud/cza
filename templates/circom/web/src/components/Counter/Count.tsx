import { Option } from '@hazae41/option'
import { watchBlockNumber } from '@wagmi/core'
import { useAtom } from 'jotai'
import { config, readCounterNumber, useReadCounterNumber } from 'l/wagmi'
import { useEffect } from 'react'
import { BarLoader } from 'react-spinners'
import { countAtom } from 's/atoms'

export const Count = () => {
  const [count, setCount] = useAtom(countAtom)
  const { data } = useReadCounterNumber()

  const unwatch = watchBlockNumber(config, {
    onBlockNumber() {
      readCounterNumber(config, {}).then((c) => {
        setCount(Option.wrap(c))
      })
    },
  })

  // biome-ignore lint/correctness/useExhaustiveDependencies: avoid infinite re renders
  useEffect(() => {
    setCount(Option.wrap(data))
    return unwatch
  }, [])

  return count.mapOrSync(<BarLoader />, (count) => <div className='font-bold text-lg'>Count: {count?.toString()}</div>)
}
