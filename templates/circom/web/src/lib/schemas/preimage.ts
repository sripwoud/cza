import { z } from 'zod'

export const preimageSchema = z.string().nonempty()
