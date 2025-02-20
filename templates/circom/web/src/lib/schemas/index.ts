import { z } from 'zod'
import { usernameSchema as username } from './username'

export const usernameSchema = z.object({ username })
export type UsernameSchema = z.infer<typeof usernameSchema>
