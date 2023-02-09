<script lang="ts">
  import { browser } from '$app/environment'
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import type { PageData } from './$types'
  import Link from './Link.svelte'
  import Pagination from './Pagination.svelte'

  const pageSize = 10

  function url(search: string, page: number) {
    const searchParams = new URLSearchParams()
    if (page > 1) {
      searchParams.set('start', ((page - 1) * pageSize).toString())
    }
    if (search) {
      searchParams.set('q', search)
    }
    const params = searchParams.toString()
    if (params !== '') {
      return `/?${params}`
    }
    return '/'
  }

  export let data: PageData
  let currentSearch = ''
  let currentPage = 1

  page.subscribe(({ url }) => {
    const startParam = url.searchParams.get('start')
    const start = startParam ? parseInt(startParam) : 0
    currentPage = Math.floor(start / pageSize) + 1
  })

  let timeout: number | undefined = undefined
  $: if (browser) {
    clearTimeout(timeout)
    timeout = window.setTimeout(async () => {
      await goto(url(currentSearch, currentPage), { replaceState: true, keepFocus: true })
    }, 500)
  }
</script>

<nav>
  <input type="text" placeholder="search" bind:value={currentSearch} />
  <span>{data.links.total} links</span>
  <a href="/new">New</a>
</nav>
<div>
  {#each data.links.items as link, i (link.id)}
    {#if i !== 0}
      <hr />
    {/if}
    <Link {link} />
  {/each}
  <hr />
  <Pagination
    page={currentPage}
    pages={Math.ceil(data.links.total / pageSize)}
    url={(newPage) => url(currentSearch, newPage)}
  />
</div>

<style>
  nav {
    margin-bottom: 20px;
    display: flex;
    align-items: center;
  }

  nav > * {
    display: inline-block;
    margin-right: 16px;
  }

  hr {
    border: 0;
  }
</style>
