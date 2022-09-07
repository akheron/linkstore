import React from 'react'
import { Link } from '../api'
import { navigate } from '../utils/location'
import { format } from 'date-fns'
import { HGap, separatedList, TextButton, VGap } from './utils'
import { useAppDispatch, useAppSelector } from '../store'
import { nextPage, prevPage, setQuery, useLinks } from '../slices/searchSlice'
import * as styles from './FrontPage.module.css'

export default React.memo(function FrontPage() {
  const dispatch = useAppDispatch()
  const searchQuery = useAppSelector((state) => state.search.query)
  const { data, isLoading, isError } = useLinks()

  if (isLoading) return <div className={styles.content}>Loading...</div>
  if (isError) return <div className={styles.content}>Error loading links</div>
  if (!data) return null

  const linkCount = data.total

  return (
    <div className={styles.content}>
      <div className={styles.searchBar}>
        <input
          type="text"
          placeholder="search"
          value={searchQuery}
          onChange={(e) => {
            dispatch(setQuery(e.target.value))
          }}
        />
        <HGap size={24} />
        <span>{linkCount} links</span>
        <HGap size={24} />
        <TextButton className={styles.new} onClick={() => navigate('/new')}>
          new
        </TextButton>
      </div>
      <VGap />
      <LinkList links={data.items} />
      <VGap />
      <Pagination />
    </div>
  )
})

const LinkList = React.memo(function LinkList({ links }: { links: Link[] }) {
  return (
    <div>
      {separatedList(
        links,
        (link) => [link.id, <LinkListItem link={link} />],
        <VGap />
      )}
    </div>
  )
})

const LinkListItem = React.memo(function Link({ link }: { link: Link }) {
  const tags = link.tags.filter((tag) => tag !== '')
  return (
    <div className={styles.link}>
      <a rel="noreferrer" href={link.href}>
        {link.description}
      </a>
      {tags.length > 0 ? (
        <div>
          {separatedList(
            tags,
            (tag, i) => [i, <span className={styles.tag}>{tag}</span>],
            <HGap size={8} />
          )}
        </div>
      ) : null}
      <div>
        <span className={styles.date}>
          {format(new Date(link.time), 'MMM d y H:mm')}
        </span>
        <HGap size={8} />
        <TextButton className={styles.action} onClick={() => undefined}>
          edit
        </TextButton>
        <HGap size={8} />
        <TextButton className={styles.action} onClick={() => undefined}>
          delete
        </TextButton>
      </div>
    </div>
  )
})

const Pagination = React.memo(function Pagination() {
  const dispatch = useAppDispatch()
  const currentPage = useAppSelector((state) => state.search.page)
  const { data, isFetching } = useLinks()

  if (!data) return null

  return (
    <div className={styles.pagination}>
      {currentPage < data.pages ? (
        <TextButton onClick={() => dispatch(nextPage())} disabled={isFetching}>
          older
        </TextButton>
      ) : null}
      {currentPage > 1 ? (
        <TextButton onClick={() => dispatch(prevPage())} disabled={isFetching}>
          newer
        </TextButton>
      ) : null}
    </div>
  )
})
