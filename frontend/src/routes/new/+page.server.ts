import { error, fail, redirect } from '@sveltejs/kit'
import { API_URL } from '$lib/server/env'
import type { Actions } from './$types'

export const actions = {
  default: async ({ request, fetch }) => {
    const form = await request.formData()
    const body = {
      href: form.get('href'),
      description: form.get('description'),
      extended: form.get('extended'),
      tags: form.get('tags')?.toString().split(/\s+/).filter(Boolean) || [],
      shared: true,
      toRead: false,
    }
    const response = await fetch(API_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })
    if (response.status === 400) return fail(400, { error: await response.text() })
    if (response.status === 201) throw redirect(303, '/')
    throw error(500, 'Something went wrong')
  },
} satisfies Actions
