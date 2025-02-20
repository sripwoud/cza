import type { router } from 'l/router'
import { Providers } from 'p'

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

export default function App() {
  return <Providers />
}
