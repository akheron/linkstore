import React, { useCallback, useState } from 'react'
import { Provider } from 'react-redux'
import { Link, useLinksQuery } from './api'
import { store } from './store'
import * as styles from './App.module.css'

export default React.memo(function App() {
  return (
    <div>
      <Provider store={store}>
        <App2 />
      </Provider>
    </div>
  )
})

const App2 = React.memo(function App2() {
  const [searchText, setSearchText] = useState('')
  const { data, isLoading, isError } = useLinksQuery({
    searchText,
    start: 1,
  })

  if (isLoading) return <div>Loading...</div>
  if (isError) return <div>Error loading links</div>
  if (!data) return null

  return (
    <div className={styles.app}>
      <SearchBar
        searchText={searchText}
        onChange={setSearchText}
        linkCount={data.total}
      />
      <VGap />
      <LinkList links={data.items} />
    </div>
  )
})

const SearchBar = React.memo(function SearchBar({
  searchText,
  onChange,
  linkCount,
}: {
  searchText: string
  onChange: (value: string) => void
  linkCount: number
}) {
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      onChange(e.target.value)
    },
    [onChange]
  )
  return (
    <div className={styles.searchBar}>
      <input
        type="text"
        placeholder="search"
        value={searchText}
        onChange={handleChange}
      />
      <HGap />
      <span>{linkCount} links</span>
    </div>
  )
})

const LinkList = React.memo(function LinkList({ links }: { links: Link[] }) {
  return (
    <div>
      {links.map((link, i) => (
        <>
          <div key={link.id} className={styles.link}>
            <a rel="noreferrer" href={link.href}>
              {link.description}
            </a>
            <div className={styles.date}>
              {link.time.toISOString()} edit delete
            </div>
          </div>
          {i !== links.length - 1 ? <VGap /> : null}
        </>
      ))}
    </div>
  )
})

const HGap = React.memo(function HGap() {
  return <div className={styles.hgap} />
})

const VGap = React.memo(function VGap() {
  return <div className={styles.vgap} />
})
