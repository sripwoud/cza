import { createLazyFileRoute } from '@tanstack/react-router'
import { UsernameForm as component } from 'c/UsernameForm'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createLazyFileRoute('/form')({ component, pendingComponent })
