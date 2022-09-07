import { useEffect, useState } from 'react'

const callbacks: (() => void)[] = []

function addCallback(cb: () => void) {
  callbacks.push(cb)
}

function removeCallback(cb: () => void) {
  const index = callbacks.indexOf(cb)
  if (index > -1) {
    callbacks.splice(index, 1)
  }
}

function invokeCallbacks() {
  callbacks.forEach((cb) => cb())
}

addEventListener('popstate', invokeCallbacks)

export function navigate(path: string) {
  history.pushState(null, '', path)
  invokeCallbacks()
}

export function useLocation(): string {
  const [, set] = useState([])
  const path = window.location.pathname

  useEffect(() => {
    const callback = () => set([])
    addCallback(callback)
    return () => {
      removeCallback(callback)
    }
  }, [])

  return path
}
