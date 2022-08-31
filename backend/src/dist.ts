import * as koa from 'koa'
import * as Router from '@koa/router'
import * as path from 'path'
import * as mount from 'koa-mount'
import * as send from 'koa-send'
import * as serve from 'koa-static'

const distPath = '../dist'

const index = (ctx: koa.Context) => send(ctx, 'index.html', { root: distPath })

export default function (app: koa) {
  const router = new Router()

  router
    .get('/', index)
    .get('/new', index)
    .get('/import', index)
    .get('/:id/edit', index)

  app.use(router.routes())
  app.use(mount('/static', serve(path.join(distPath, 'static'))))
}
