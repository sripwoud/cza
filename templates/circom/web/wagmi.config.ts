import { defineConfig } from '@wagmi/cli'
import { actions, foundry, react } from '@wagmi/cli/plugins'
import * as chains from 'wagmi/chains'

export default defineConfig({
  out: 'src/lib/wagmi/generated.ts',
  plugins: [
    actions(),
    foundry({
      deployments: {
        Counter: {
          [chains.foundry.id]: '0x5FbDB2315678afecb367f032d93F642f64180aa3',
          [chains.sepolia.id]: '0xFd4364354145989210ee6d93a36B4eC7D8eC65EA',
        },
      },
      project: '../contracts',
    }),
    react(),
  ],
})
