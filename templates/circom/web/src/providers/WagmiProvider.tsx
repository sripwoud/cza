import { config } from 'l/wagmi'
import type { ReactNode } from 'react'
import { WagmiProvider as _WagmiProvider } from 'wagmi'

export function WagmiProvider({ children }: { children: ReactNode }) {
  return <_WagmiProvider config={config}>{children}</_WagmiProvider>
}
