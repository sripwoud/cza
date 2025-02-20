import { createConfig, http } from 'wagmi'
import { anvil, sepolia } from 'wagmi/chains'

declare module 'wagmi' {
  interface Register {
    config: typeof config
  }
}
export const config = createConfig({
  chains: [anvil, sepolia],
  multiInjectedProviderDiscovery: false,
  transports: {
    [anvil.id]: http(),
    [sepolia.id]: http(),
  },
})
