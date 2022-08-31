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

const pageSize = 20

export const api = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({ baseUrl: '/' }),
  endpoints: (builder) => ({
    links: builder.query<LinksResponse, { searchText: string; start: number }>({
      query: (arg) =>
        `/api/?search=${encodeURIComponent(arg.searchText)}&start=${
          arg.start
        }&count=${pageSize}`,
      transformResponse: (data: JsonOf<LinksResponse>) => ({
        ...data,
        items: data.items.map((item) => ({
          ...item,
          time: new Date(item.time),
        })),
      }),
    }),
  }),
})

export const { useLinksQuery } = api
