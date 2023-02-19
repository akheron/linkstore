import basicAuth from 'basic-auth'
import pg from 'pg'
import { Config } from './lib/server/schemas'
import { DB } from './lib/server/db'
import type { Handle } from '@sveltejs/kit'

let _config: Config | undefined = undefined

function config(): Config {
  if (!_config) {
    _config = Config.parse(process.env)
  }
  return _config
}

function checkAuth(auth: string): boolean {
  const { BASIC_AUTH_USER, BASIC_AUTH_PASSWORD } = config()
  if (!BASIC_AUTH_USER || !BASIC_AUTH_PASSWORD) return true

  const result = basicAuth.parse(auth)
  if (!result) return false
  return result.name === BASIC_AUTH_USER && result.pass === BASIC_AUTH_PASSWORD
}

let _db: DB | undefined = undefined

function db() {
  if (!_db) {
    const { DATABASE_URL, DATABASE_SSL } = config()
    _db = new DB(
      new pg.Pool({
        connectionString: DATABASE_URL,
        ssl: DATABASE_SSL ? { rejectUnauthorized: false } : undefined,
      })
    )
  }
  return _db
}

export const handle = (({ event, resolve }) => {
  const auth = event.request.headers.get('Authorization')
  if (!checkAuth(auth ?? '')) {
    return new Response('Unauthorized', { status: 401, headers: { 'WWW-Authenticate': 'Basic' } })
  }

  event.locals.db = db()

  return resolve(event)
}) satisfies Handle
