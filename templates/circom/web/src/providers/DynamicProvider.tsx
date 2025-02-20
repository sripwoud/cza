import { EthereumWalletConnectors } from '@dynamic-labs/ethereum'
import { DynamicContextProvider } from '@dynamic-labs/sdk-react-core'
import { config } from 'l/config'
import type { ReactNode } from 'react'

export function DynamicProvider({ children }: { children: ReactNode }) {
  return (
    <DynamicContextProvider
      settings={{
        enableVisitTrackingOnConnectOnly: false,
        environmentId: config.dynamic.environmentId,
        initialAuthenticationMode: 'connect-only',
        walletConnectors: [EthereumWalletConnectors],
      }}
    >
      {children}
    </DynamicContextProvider>
  )
}
