import * as t from 'io-ts'
import { IntFromString } from 'io-ts-types/lib/IntFromString'

export const nothing = t.undefined

export const linkBody = t.type({
  href: t.string,
  description: t.string,
  extended: t.string,
  shared: t.boolean,
  toRead: t.boolean,
  tags: t.array(t.string),
})

export const paginatedSearch = t.partial({
  q: t.string,
  start: IntFromString,
  count: IntFromString,
})

export const id = t.type({
  id: IntFromString,
})
