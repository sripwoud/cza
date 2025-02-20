import { createLazyFileRoute } from '@tanstack/react-router'
import { Home as component } from 'c/Home'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createLazyFileRoute('/')({ component, pendingComponent })
