import { Route } from 'r/query'
import type { FC } from 'react'

interface UserProps {
  id: number
  name: string
}

const User: FC<Pick<UserProps, 'name'>> = ({ name }) => <li>{name}</li>

export function Users() {
  const users: UserProps[] = Route.useLoaderData()

  return (
    <ul className='list-disc list-inside'>
      {users.map(({ id, name }) => <User key={id} name={name} />)}
    </ul>
  )
}
