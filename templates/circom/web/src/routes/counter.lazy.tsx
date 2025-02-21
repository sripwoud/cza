import { createLazyFileRoute } from '@tanstack/react-router'
import { Counter as component } from 'c/Counter'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createLazyFileRoute('/counter')({
  component,
  pendingComponent,
})
