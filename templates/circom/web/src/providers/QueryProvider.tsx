import { QueryClientProvider } from '@tanstack/react-query'
import { query as client } from 'l/query'
import type { ReactNode } from 'react'

export function QueryProvider({ children }: { children: ReactNode }) {
  return <QueryClientProvider client={client}>{children}</QueryClientProvider>
}
