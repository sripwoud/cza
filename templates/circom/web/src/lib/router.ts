import { createRouter } from '@tanstack/react-router'
import { NotFound as defaultNotFoundComponent } from 'components/NotFound'
import { routeTree } from 'routeTree.gen'

export const router = createRouter({
  defaultNotFoundComponent,
  routeTree,
})
