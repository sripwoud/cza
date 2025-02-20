import tailwindcss from '@tailwindcss/vite'
import { TanStackRouterVite } from '@tanstack/router-plugin/vite'
import react from '@vitejs/plugin-react-swc'
import { defineConfig } from 'vite'
import { nodePolyfills } from 'vite-plugin-node-polyfills'
import tsconfigPaths from 'vite-tsconfig-paths'

const plugins = [
  // alternative workaround to https://docs.dynamic.xyz/troubleshooting/react/vitejs-polyfills-necessary-for-dynamic-sdk
  nodePolyfills({
    globals: {
      process: true,
    },
  }),
  react(),
  tailwindcss(),
  TanStackRouterVite(),
  tsconfigPaths(),
]

export default defineConfig(({ mode }) => {
  return {
    plugins,
    server: { open: mode === 'development' },
  }
})
