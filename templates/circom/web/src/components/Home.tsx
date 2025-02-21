import { config } from 'l/config'

export function Home() {
  return <div>Hello {config.appName}!</div>
}
