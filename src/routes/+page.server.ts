import { error } from '@sveltejs/kit'
import type { LinkData } from '$lib/types'
import { Id, PaginatedSearch } from '$lib/server/schemas'
import { parse } from '$lib/server/utils'
import type { Actions, PageServerLoad } from './$types'

export const load = (async ({ url, locals: { db } }) => {
  const pageSize = 20
  const { q, page = 1 } = parse(PaginatedSearch, Object.fromEntries(url.searchParams))
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
    const data = parse(Id, Object.fromEntries((await request.formData()).entries()))
    if (!(await db.deleteLink(data.id))) {
      throw error(404)
    }
  },
} satisfies Actions
