import { createLazyFileRoute } from '@tanstack/react-router'
import { Latom as component } from 'c/Latom'
import { BarLoader as pendingComponent } from 'react-spinners'

export const Route = createLazyFileRoute('/state')({ component, pendingComponent })
