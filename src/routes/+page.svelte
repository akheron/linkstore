<script lang="ts">
  import type { FormEventHandler } from 'svelte/elements'
  import { goto } from '$app/navigation'
  import type { PageData } from './$types'
  import Link from './Link.svelte'
  import Pagination from './Pagination.svelte'

  export let data: PageData
  let searchText = ''

  const url = (search: string, page: number) => {
    const searchParams = new URLSearchParams()
    if (page > 1) {
      searchParams.set('page', page.toString())
    }
    if (search) {
      searchParams.set('q', search)
    }
    const params = searchParams.toString()
    return params !== '' ? `/?${params}` : '/'
  }

  let timeout: number | undefined = undefined
  const onSearchInput: FormEventHandler<HTMLInputElement> = (e) => {
    searchText = e.currentTarget.value

    clearTimeout(timeout)
    timeout = window.setTimeout(async () => {
      await goto(url(searchText, 1), { replaceState: true, keepFocus: true })
    }, 500)
  }
</script>

<nav>
  <input type="text" placeholder="search" on:input={onSearchInput} />
  <span>{data.total} links</span>
  <a href="/new">New</a>
</nav>
<div>
  {#each data.links as link, i (link.id)}
    {#if i !== 0}
      <hr />
    {/if}
    <Link {link} />
  {/each}
  <hr />
  <Pagination page={data.page} pages={data.pages} url={(newPage) => url(searchText, newPage)} />
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
