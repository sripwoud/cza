import { z } from 'zod'
import { preimageSchema as preimage } from './preimage'
import { usernameSchema as username } from './username'

export const usernameSchema = z.object({ username })
export type UsernameSchema = z.infer<typeof usernameSchema>

export const preimageSchema = z.object({ preimage })
export type PreimageSchema = z.infer<typeof preimageSchema>
