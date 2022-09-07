import { configureStore } from '@reduxjs/toolkit'
import { setupListeners } from '@reduxjs/toolkit/query/react'
import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux'
import * as searchSlice from './slices/searchSlice'
import { api } from './api'
import { listenerMiddleware } from './listenerMiddleware'

export const store = configureStore({
  reducer: {
    [api.reducerPath]: api.reducer,
    search: searchSlice.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .prepend(listenerMiddleware.middleware)
      .concat(api.middleware),
})

setupListeners(store.dispatch)

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch

export const useAppDispatch: () => AppDispatch = useDispatch
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector
