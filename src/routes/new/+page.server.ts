import { fail, redirect } from '@sveltejs/kit'
import type { Actions } from './$types'
import { LinkBody } from '../../lib/server/schemas'

export const actions = {
  default: async ({ request, locals: { db } }) => {
    const form = await request.formData()
    const body = LinkBody.safeParse({
      href: form.get('href'),
      description: form.get('description'),
      extended: form.get('extended'),
      tags: form.get('tags')?.toString().split(/\s+/).filter(Boolean) || [],
      shared: true,
      toRead: false,
    })
    if (!body.success) {
      return fail(400, {
        error: body.error.issues.map((issue) => `${issue.path}: ${issue.message}`).join(', '),
      })
    }
    const result = await db.createLink(body.data)
    if (result.type === 'AlreadyExists') {
      return fail(400, { error: 'Link already exists' })
    }
    throw redirect(303, '/')
  },
} satisfies Actions
