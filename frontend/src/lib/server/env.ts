import { env } from '$env/dynamic/private'

function ensureEnv(key: string): string {
  const value = env[key]
  if (value === undefined) {
    console.log(process.env)
    throw new Error(`Missing environment variable: ${key}`)
  }
  return value
}

export const API_URL = ensureEnv('API_URL')
