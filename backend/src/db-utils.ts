import { QueryResult } from 'pg'

export const allRows = async <A>(query: Promise<QueryResult>): Promise<A[]> =>
  (await query).rows

export const singleRowOr = async <E, A>(
  defaultValue: E,
  makeValue: (row: any) => A,
  query: Promise<QueryResult>
): Promise<E | A> => {
  const rows = (await query).rows
  if (rows.length === 1) {
    return makeValue(rows[0])
  }
  return defaultValue
}

export const singleRowOrError = async <A>(
  makeValue: (row: any) => A,
  query: Promise<QueryResult>
): Promise<A> => {
  const rows = (await query).rows
  if (rows.length !== 1) {
    throw new Error(`Expected a single row but got ${rows.length}`)
  }
  return makeValue(rows[0])
}

export const onIntegrityError = async <E, A>(
  errorValue: E,
  query: Promise<A>
): Promise<E | A> => {
  try {
    return await query
  } catch (error) {
    if (error.code && isIntegrityError(error.code)) {
      return errorValue
    }
    throw error
  }
}

const isIntegrityError = (sqlState: string): boolean =>
  sqlState.indexOf('23') === 0
