import { error } from '@sveltejs/kit'
import type { LinkData } from '$lib/types'
import { Id, PaginatedSearch } from '$lib/server/schemas'
import { parse } from '$lib/server/utils'
import type { Actions, PageServerLoad } from './$types'

interface LinksResponse {
  items: LinkData[]
  start: number
  total: number
}

export const load = (async ({ url, locals: { db } }) => {
  const { q, start, count } = parse(PaginatedSearch, Object.fromEntries(url.searchParams))
  const total = await db.linkCount(q)
  const items = await db.searchLinks(q, start, count)
  return { links: { items, start, total } }
}) satisfies PageServerLoad<{ links: LinksResponse }>

export const actions = {
  deleteLink: async ({ request, locals: { db } }) => {
    const data = parse(Id, Object.fromEntries((await request.formData()).entries()))
    if (!(await db.deleteLink(data.id))) {
      throw error(404)
    }
  },
} satisfies Actions
