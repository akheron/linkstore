import { error } from '@sveltejs/kit'
import type { z } from 'zod'

export function parse<T>(codec: z.ZodType<T, z.ZodTypeDef, unknown>, data: unknown): T {
  const result = codec.safeParse(data)
  if (!result.success) {
    throw error(400, {
      message: result.error.issues.map((issue) => `${issue.path}: ${issue.message}`).join(', '),
    })
  }
  return result.data
}
