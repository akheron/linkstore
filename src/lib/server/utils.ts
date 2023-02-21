import type { z } from 'zod'

export function stringifyError(error: z.ZodError): string {
  return error.issues.map((issue) => `${issue.path}: ${issue.message}`).join(', ')
}

export function parseOrDefault<T>(
  codec: z.ZodType<T, z.ZodTypeDef, unknown>,
  default_: T,
  data: unknown
): T {
  const result = codec.safeParse(data)
  return result.success ? result.data : default_
}
