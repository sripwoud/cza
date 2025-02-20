import { createFileRoute } from '@tanstack/react-router'
import { Counter as component } from 'c/Counter'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createFileRoute('/web3')({
  component,
  pendingComponent,
})
