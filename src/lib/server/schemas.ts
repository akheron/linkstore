import { z } from 'zod'

export const Config = z.object({
  DATABASE_URL: z.string(),
  DATABASE_SSL: z.boolean().optional(),
  BASIC_AUTH_USER: z.string().optional(),
  BASIC_AUTH_PASSWORD: z.string().optional(),
})
export type Config = z.infer<typeof Config>

export const LinkBody = z.object({
  href: z.string().url(),
  description: z.string(),
  extended: z.string(),
  shared: z.boolean(),
  toRead: z.boolean(),
  tags: z.array(z.string()),
})
export type LinkBody = z.infer<typeof LinkBody>

export const PaginatedSearch = z.object({
  q: z.string().optional(),
  page: z.coerce.number().int().min(1).optional(),
})

export const Id = z.object({ id: z.coerce.number().int().min(1) })
