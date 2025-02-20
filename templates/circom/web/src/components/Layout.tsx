import { Outlet } from '@tanstack/react-router'
import { ReactQueryDevtools, TanStackRouterDevtools } from 'c/DevTools'
import { Footer } from 'c/Footer'
import { Header } from 'c/Header'

export function Layout() {
  return (
    <div className='flex flex-col min-h-screen px-6 pt-4'>
      <Header />
      <main className='flex-1 overflow-hidden pt-4'>
        <Outlet />
      </main>
      <ReactQueryDevtools />
      <TanStackRouterDevtools position='bottom-left' />
      <Footer />
    </div>
  )
}
