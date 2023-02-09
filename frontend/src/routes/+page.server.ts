import type { LinkData } from './types'
import type { Actions, PageServerLoad } from './$types'
import { API_URL } from '$lib/server/env'
import { error } from '@sveltejs/kit'

interface LinksResponse {
  items: LinkData[]
  start: number
  total: number
}

export const load = (async ({ url, fetch }) => {
  const links: LinksResponse = await fetch(`${API_URL}?${url.searchParams}`).then((res) =>
    res.json()
  )
  return { links }
}) satisfies PageServerLoad<{ links: LinksResponse }>

export const actions = {
  deleteLink: async ({ request, fetch }) => {
    const form = await request.formData()
    const id = form.get('id')
    const response = await fetch(`${API_URL}/${id}`, { method: 'DELETE' })
    if (!response.ok) throw error(500, 'Something went wrong')
  },
} satisfies Actions
