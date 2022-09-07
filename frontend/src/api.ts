import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'

export type JsonOf<T> = T extends string | number | boolean | null | undefined
  ? T
  : T extends Date
  ? string
  : T extends Array<infer U>
  ? Array<JsonOf<U>>
  : { [P in keyof T]: JsonOf<T[P]> }

export interface Link {
  id: number
  href: string
  description: string
  extended: string
  time: Date
  shared: boolean
  toRead: boolean
  tags: string[]
}

export interface LinksResponse {
  items: Link[]
  start: number
  total: number
}

export const api = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({ baseUrl: '/' }),
  endpoints: (builder) => ({
    links: builder.query<
      LinksResponse,
      { query: string; start: number; pageSize: number }
    >({
      query: (arg) =>
        `/api/?q=${encodeURIComponent(arg.query)}&start=${arg.start}&count=${
          arg.pageSize
        }`,
    }),
  }),
})

export const { useLinksQuery } = api
