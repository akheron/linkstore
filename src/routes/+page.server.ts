import { fail } from '@sveltejs/kit'
import type { LinkData } from '$lib/types'
import { Id, PaginatedSearch } from '$lib/server/schemas'
import { parseOrDefault, stringifyError } from '$lib/server/utils'
import type { Actions, PageServerLoad } from './$types'

export const load = (async ({ url, locals: { db } }) => {
  const pageSize = 20
  const { q, page = 1 } = parseOrDefault(
    PaginatedSearch,
    { q: '', page: 1 },
    Object.fromEntries(url.searchParams)
  )
  const start = (page - 1) * pageSize

  const total = await db.linkCount(q)
  const links = await db.searchLinks(q, start, pageSize)

  return { links, total, page, pages: Math.ceil(total / pageSize) }
}) satisfies PageServerLoad<{
  links: LinkData[]
  total: number
  page: number
  pages: number
}>

export const actions = {
  deleteLink: async ({ request, locals: { db } }) => {
    const input = Object.fromEntries((await request.formData()).entries())
    const result = Id.safeParse(input)
    if (!result.success) {
      return fail(400, { message: stringifyError(result.error) })
    }
    if (!(await db.deleteLink(result.data.id))) {
      return fail(404, { message: 'No such link' })
    }
  },
} satisfies Actions
