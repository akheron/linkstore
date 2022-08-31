import * as t from 'io-ts'
import * as Either from 'fp-ts/lib/Either'
import { PathReporter } from 'io-ts/lib/PathReporter'
import { IntFromString } from 'io-ts-types/lib/IntFromString'
import { BooleanFromString } from 'io-ts-types'

const codec = t.intersection([
  t.type({
    BIND_HOST: t.string,
    PORT: IntFromString,
    DATABASE_URL: t.string,
  }),
  t.partial({
    DATABASE_SSL: BooleanFromString,
    USERNAME: t.string,
    PASSWORD: t.string,
  }),
])

interface Config {
  bindHost: string
  port: number
  basicAuth: {
    name: string
    pass: string
  } | null
  databaseUrl: string
  databaseSsl: boolean
}

function configFromEnv(env: t.TypeOf<typeof codec>): Config {
  return {
    bindHost: env.BIND_HOST,
    port: env.PORT,
    basicAuth:
      env.USERNAME && env.PASSWORD
        ? {
            name: env.USERNAME,
            pass: env.PASSWORD,
          }
        : null,
    databaseUrl: env.DATABASE_URL,
    databaseSsl: env.DATABASE_SSL ?? true,
  }
}

const result = codec.decode(process.env)
if (Either.isLeft(result)) {
  throw new Error(
    `Invalid environment variables: ${PathReporter.report(result)}`
  )
}

export default configFromEnv(result.right)
