import { Pool } from 'pg'

import { Link, LinkBody } from './types'
import {
  allRows,
  singleRowOrError,
  onIntegrityError,
  singleRowOr,
} from './db-utils'
import config from './config'

const pool = new Pool({
  connectionString: config.databaseUrl,
  ssl: config.databaseSsl ? { rejectUnauthorized: false } : undefined,
})

interface Success<A> {
  type: 'Success'
  value: A
}
interface NotFound {
  type: 'NotFound'
}
interface AlreadyExists {
  type: 'AlreadyExists'
}

const success = <A>(value: A): Success<A> => ({ type: 'Success', value })
const notFound: NotFound = { type: 'NotFound' }
const alreadyExists: AlreadyExists = { type: 'AlreadyExists' }

export const linkCount = async (
  searchText: string | undefined
): Promise<number> => {
  const { where, params } = searchWhere(searchText, 1)
  const sql = `
SELECT count(*)::int as count
FROM links ${where}
`
  return (await singleRowOrError((x) => x, pool.query(sql, params))).count
}

export const searchLinks = async (
  searchText: string | undefined,
  start: number,
  count: number
): Promise<Link[]> => {
  const { where, params } = searchWhere(searchText, 3)
  const sql = `
SELECT
    id,
    href,
    description,
    extended,
    time,
    shared,
    toRead,
    tags
FROM links
${where}
ORDER BY time DESC
OFFSET $1 LIMIT $2
`
  return allRows(pool.query(sql, [start, count, ...params]))
}

const searchWhere = (
  searchText: string | undefined,
  firstParamIndex: number
): { where: string; params: string[] } => {
  const searchTerms = searchText ? searchText.split(/\s+/) : []
  if (searchTerms.length === 0) return { where: '', params: [] }
  return {
    where:
      'WHERE ' +
      searchTerms
        .map((_, index) => {
          const p = index * 2 + firstParamIndex
          return `
(href LIKE $${p} OR
 description LIKE $${p} OR
 extended LIKE $${p} OR
 (SELECT bool_or (unnest LIKE $${p + 1}) FROM unnest(tags)))
`
        })
        .join(' AND '),
    params: searchTerms.map((term) => [`%${term}%`, `${term}%`]).flat(),
  }
}

export const readLink = async (id: number): Promise<Link | null> =>
  singleRowOr(
    null,
    (row) => row,
    pool.query(
      `
SELECT
    id,
    href,
    description,
    extended,
    time,
    shared,
    toRead,
    tags
FROM links
WHERE id = $1
`,
      [id]
    )
  )

export type CreateResult<A> = Success<A> | AlreadyExists

export const createLink = (body: LinkBody): Promise<CreateResult<Link>> =>
  onIntegrityError(
    alreadyExists,
    singleRowOrError(
      success,
      pool.query(
        `
INSERT INTO links (
    href,
    description,
    extended,
    time,
    shared,
    toRead,
    tags
)
VALUES ($1, $2, $3, current_timestamp, $4, $5, $6)
RETURNING
    id,
    href,
    description,
    extended,
    time,
    shared,
    toRead,
    tags
`,
        [
          body.href,
          body.description,
          body.extended,
          body.shared,
          body.toRead,
          body.tags,
        ]
      )
    )
  )

export type UpdateResult<A> = Success<A> | NotFound | AlreadyExists

export const updateLink = (
  id: number,
  body: LinkBody
): Promise<UpdateResult<Link>> =>
  onIntegrityError(
    alreadyExists,
    singleRowOr(
      notFound,
      success,
      pool.query(
        `
UPDATE links
SET
    href = $1,
    description = $2,
    extended = $3,
    shared = $4,
    toRead = $5,
    tags = $6
WHERE
    id = $7
RETURNING
    id,
    href,
    description,
    extended,
    time,
    shared,
    toRead,
    tags
`,
        [
          body.href,
          body.description,
          body.extended,
          body.shared,
          body.toRead,
          body.tags,
          id,
        ]
      )
    )
  )

export const deleteLink = async (id: number): Promise<boolean> => {
  const result = await pool.query(
    `
DELETE FROM links
WHERE id = $1
`,
    [id]
  )
  return result.rowCount === 1
}
