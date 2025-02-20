import { RouterProvider as RouterProviderBase } from '@tanstack/react-router'
import { router } from 'l/router'

export function RouterProvider() {
  return <RouterProviderBase router={router} />
}
