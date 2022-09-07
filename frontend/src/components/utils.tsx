import React, { Fragment, ReactNode } from 'react'
import * as styles from './utils.module.css'

export const TextButton = React.memo(function TextButton({
  className,
  children,
  ...props
}: {
  onClick: () => void
  className?: string
  disabled?: boolean
  children?: React.ReactNode
}) {
  const classes = [styles.textButton, className].filter((x) => !!x).join(' ')
  return (
    <button className={classes} {...props}>
      {children}
    </button>
  )
})

export const HGap = React.memo(function HGap({ size = 32 }: { size?: number }) {
  return <span style={{ display: 'inline-block', marginRight: size }} />
})

export const VGap = React.memo(function VGap({ size = 32 }: { size?: number }) {
  return <div style={{ marginBottom: size }} />
})

export function separatedList<T>(
  items: T[],
  mapFn: (item: T, index: number) => [string | number, ReactNode],
  separator: ReactNode
) {
  return items.reduce<ReactNode[]>((acc, item, i) => {
    const [key, node] = mapFn(item, i)
    if (i === 0) return [<Fragment key={key}>{node}</Fragment>]
    return [
      ...acc,
      <Fragment key={key}>
        {separator}
        {node}
      </Fragment>,
    ]
  }, [])
}
