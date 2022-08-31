require('dotenv').config()

import * as koa from 'koa'
import * as bodyParser from 'koa-bodyparser'
import * as mount from 'koa-mount'
import basicAuth = require('koa-basic-auth')

import config from './config'
import api from './api'
import dist from './dist'

const app = new koa()

if (config.basicAuth) {
  app.use(basicAuth(config.basicAuth))
}
app.use(bodyParser())
dist(app)
app.use(mount('/api', api))

app.listen(config.port, config.bindHost, () => {
  console.log(`Listening on port ${config.port}`)
})
