import { DynamicWidget } from '@dynamic-labs/sdk-react-core'
import { Link } from '@tanstack/react-router'
import { config } from 'l/config'
import { capitalize } from 'l/format'

const links = ['counter', 'form', 'proof', 'query', 'state'] as const

export function Header() {
  return (
    <nav
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
      }}
    >
      <Link to='/'>
        <h1 className='text-xl font-bold'>{config.appName}</h1>
      </Link>
      {links.map((r) => (
        <Link key={r} to={`/${r}`}>
          {capitalize(r)}
        </Link>
      ))}
      <DynamicWidget />
    </nav>
  )
}
