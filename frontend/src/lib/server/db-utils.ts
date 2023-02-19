import type { QueryResult } from 'pg'

export interface Success<A> {
  type: 'Success'
  value: A
}

export interface NotFound {
  type: 'NotFound'
}

export interface AlreadyExists {
  type: 'AlreadyExists'
}

export const success = <A>(value: A): Success<A> => ({ type: 'Success', value })
export const notFound: NotFound = { type: 'NotFound' }
export const alreadyExists: AlreadyExists = { type: 'AlreadyExists' }

export async function allRows<A>(query: Promise<QueryResult>): Promise<A[]> {
  return (await query).rows
}

export async function singleRowOr<E, A>(
  defaultValue: E,
  makeValue: (row: any) => A,
  query: Promise<QueryResult>
): Promise<E | A> {
  const rows = (await query).rows
  if (rows.length === 1) {
    return makeValue(rows[0])
  }
  return defaultValue
}

export async function singleRowOrError<A>(
  makeValue: (row: any) => A,
  query: Promise<QueryResult>
): Promise<A> {
  const rows = (await query).rows
  if (rows.length !== 1) {
    throw new Error(`Expected a single row but got ${rows.length}`)
  }
  return makeValue(rows[0])
}

export async function onIntegrityError<E, A>(errorValue: E, query: Promise<A>): Promise<E | A> {
  try {
    return await query
  } catch (error: any) {
    if (error.code && isIntegrityError(error.code)) {
      return errorValue
    }
    throw error
  }
}

const isIntegrityError = (sqlState: string): boolean => sqlState.indexOf('23') === 0
