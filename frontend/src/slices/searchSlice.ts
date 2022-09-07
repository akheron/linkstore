import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { useAppSelector } from '../store'
import { useLinksQuery } from '../api'
import { listenerMiddleware } from '../listenerMiddleware'

const pageSize = 20

const searchSlice = createSlice({
  name: 'search',
  initialState: {
    query: '',
    debouncedQuery: '',
    page: 1,
  },
  reducers: {
    setQuery: (state, action: PayloadAction<string>) => {
      state.query = action.payload
    },
    setDebouncedQuery: (state, action: PayloadAction<string>) => {
      state.debouncedQuery = action.payload
      state.page = 1
    },
    nextPage: (state) => {
      state.page += 1
    },
    prevPage: (state) => {
      if (state.page > 1) {
        state.page -= 1
      }
    },
  },
})

export const { reducer, actions } = searchSlice
export const { setQuery, nextPage, prevPage } = actions

// Delay fetching links from API by 500ms
listenerMiddleware.startListening({
  actionCreator: setQuery,
  effect: async (action, { cancelActiveListeners, delay, dispatch }) => {
    cancelActiveListeners()
    await delay(500)
    dispatch(actions.setDebouncedQuery(action.payload))
  },
})

export function useLinks() {
  const debouncedQuery = useAppSelector((state) => state.search.debouncedQuery)
  const page = useAppSelector((state) => state.search.page)
  const { data: originalData, ...rest } = useLinksQuery({
    query: debouncedQuery,
    start: (page - 1) * pageSize,
    pageSize,
  })

  const data = originalData
    ? {
        ...originalData,
        pages: Math.ceil(originalData.total / pageSize),
      }
    : null

  return { data, ...rest }
}
