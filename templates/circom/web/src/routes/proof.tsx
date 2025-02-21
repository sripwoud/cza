import { createFileRoute } from '@tanstack/react-router'
import { Proof as component } from 'c/Proof'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createFileRoute('/proof')({ component, pendingComponent })
