import { Parser, Response, Route, route, router } from 'typera-koa'

import * as db from './db'
import { Link } from './types'
import { id, linkBody, paginatedSearch } from './codecs'

type PaginatedResult<T> = {
  items: T[]
  start: number
  total: number
}

const sanitizePagination = (params: {
  start?: number
  count?: number
}): [number, number] => {
  let { start = 0, count = 10 } = params
  if (start < 0) start = 0
  if (count < 0 || count > 1000) count = 10
  return [start, count]
}

const list: Route<
  Response.Ok<PaginatedResult<Link>> | Response.BadRequest<string>
> = route
  .get('/')
  .use(Parser.query(paginatedSearch))
  .handler(async (request) => {
    const [start, count] = sanitizePagination(request.query)
    const total = await db.linkCount(request.query.q)
    const links = await db.searchLinks(request.query.q, start, count)
    return Response.ok({ start, total, items: links })
  })

const create: Route<
  Response.Created<Link> | Response.BadRequest<string>
> = route
  .post('/')
  .use(Parser.body(linkBody))
  .handler(async (request) => {
    const result = await db.createLink(request.body)
    switch (result.type) {
      case 'AlreadyExists':
        return Response.badRequest('A link with this href already exists')

      case 'Success':
        return Response.created(result.value)
    }
  })

const read: Route<Response.Ok<Link> | Response.NotFound> = route
  .get('/:id(int)')
  .handler(async (request) => {
    const link = await db.readLink(request.routeParams.id)
    if (link) {
      return Response.ok(link)
    } else {
      return Response.notFound()
    }
  })

const update: Route<
  Response.Ok<Link> | Response.NotFound | Response.BadRequest<string>
> = route
  .put('/:id(int)')
  .use(Parser.body(linkBody))
  .handler(async (request) => {
    const result = await db.updateLink(request.routeParams.id, request.body)
    switch (result.type) {
      case 'NotFound':
        return Response.notFound()
      case 'AlreadyExists':
        return Response.badRequest('A link with this href already exists')
      case 'Success':
        return Response.ok(result.value)
    }
  })

const delete_: Route<Response.NoContent | Response.NotFound> = route
  .delete('/:id(int)')
  .handler(async (request) => {
    const result = await db.deleteLink(request.routeParams.id)
    if (result) {
      return Response.noContent()
    } else {
      return Response.notFound()
    }
  })

export default router(list, create, read, update, delete_).handler()
