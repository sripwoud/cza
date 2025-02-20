import { DynamicWagmiConnector } from '@dynamic-labs/wagmi-connector'
import { DynamicProvider } from './DynamicProvider'
import { QueryProvider } from './QueryProvider'
import { RouterProvider } from './RouterProvider'
import { WagmiProvider } from './WagmiProvider'

export function Providers() {
  return (
    <DynamicProvider>
      <WagmiProvider>
        <QueryProvider>
          <DynamicWagmiConnector>
            <RouterProvider />
          </DynamicWagmiConnector>
        </QueryProvider>
      </WagmiProvider>
    </DynamicProvider>
  )
}
