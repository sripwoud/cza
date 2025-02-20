import { createFileRoute } from '@tanstack/react-router'
import { Users as component } from 'c/Users'
import { query } from 'l/query'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createFileRoute('/query')({
  component,
  loader: async () =>
    query.fetchQuery({
      queryKey: ['users'],
      queryFn: () => fetch('https://jsonplaceholder.typicode.com/users').then((r) => r.json()),
    }),
  pendingComponent,
})
