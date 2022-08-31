import * as t from 'io-ts'
import * as codecs from './codecs'

export type LinkBody = t.TypeOf<typeof codecs.linkBody>

export interface Link extends LinkBody {
  id: number
  time: string
}

export type PaginatedSearch = t.TypeOf<typeof codecs.paginatedSearch>

export type ID = t.TypeOf<typeof codecs.id>
