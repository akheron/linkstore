import type pg from 'pg'
import type { LinkData } from '../types'
import {
  allRows,
  alreadyExists,
  notFound,
  singleRowOrError,
  success,
  type AlreadyExists,
  type NotFound,
  type Success,
  onIntegrityError,
  singleRowOr,
} from './db-utils'
import type { LinkBody } from './schemas'

export class DB {
  constructor(private pool: pg.Pool) {}

  async linkCount(searchText: string | undefined): Promise<number> {
    const { where, params } = searchWhere(searchText, 1)
    const sql = `
  SELECT count(*)::int AS count
  FROM links ${where}
    `
    return (await singleRowOrError((x) => x as { count: number }, this.pool.query(sql, params)))
      .count
  }

  async searchLinks(
    searchText: string | undefined,
    start: number,
    count: number
  ): Promise<LinkData[]> {
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
FROM links ${where}
ORDER BY time DESC
OFFSET $1 LIMIT $2
    `
    return allRows(this.pool.query(sql, [start, count, ...params]))
  }

  createLink(body: LinkBody): Promise<Success<LinkData> | AlreadyExists> {
    return onIntegrityError(
      alreadyExists,
      singleRowOrError(
        success,
        this.pool.query(
          `
INSERT INTO links (href, description, extended, time, shared, toRead, tags)
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
          [body.href, body.description, body.extended, body.shared, body.toRead, body.tags]
        )
      )
    )
  }

  updateLink(id: number, body: LinkBody): Promise<Success<LinkData> | NotFound | AlreadyExists> {
    return onIntegrityError(
      alreadyExists,
      singleRowOr(
        notFound,
        success,
        this.pool.query(
          `
              UPDATE links
              SET href        = $1,
                  description = $2,
                  extended    = $3,
                  shared      = $4,
                  toRead      = $5,
                  tags        = $6
              WHERE id = $7
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
          [body.href, body.description, body.extended, body.shared, body.toRead, body.tags, id]
        )
      )
    )
  }

  async deleteLink(id: number): Promise<boolean> {
    const result = await this.pool.query(
      `
DELETE
FROM links
WHERE id = $1
      `,
      [id]
    )
    return result.rowCount === 1
  }
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
    (SELECT bool_or(unnest LIKE $${p + 1}) FROM unnest(tags)))
`
        })
        .join(' AND '),
    params: searchTerms.map((term) => [`%${term}%`, `${term}%`]).flat(),
  }
}
