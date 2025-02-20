import { lazy } from 'react'

export const TanStackRouterDevtools = process.env.NODE_ENV === 'production'
  ? () => null
  : lazy(() =>
    import('@tanstack/router-devtools').then((res) => ({
      default: res.TanStackRouterDevtools,
      // For Embedded Mode
      // default: res.TanStackRouterDevtoolsPanel
    }))
  )

export const ReactQueryDevtools = process.env.NODE_ENV === 'production'
  ? () => null
  : lazy(() =>
    import('@tanstack/react-query-devtools').then((res) => ({
      default: res.ReactQueryDevtools,
    }))
  )
