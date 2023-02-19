<script lang="ts">
  import { enhance, type SubmitFunction } from '$app/forms'
  import format from 'date-fns/format/index.js'
  import type { LinkData } from '$lib/types'

  export let link: LinkData

  const confirmDelete: SubmitFunction = ({ cancel }) => {
    if (!confirm('Are you sure you want to delete this link?')) {
      cancel()
    }
  }
</script>

<div class="root">
  <div>
    <a href={link.href} target="_blank" rel="noopener noreferrer">
      {link.description}
    </a>
  </div>
  {#if link.tags.filter(Boolean).length > 0}
    <div>
      {#each link.tags as tag}
        <span class="tag">{tag}</span>
      {/each}
    </div>
  {/if}
  <div>
    <span class="time">
      {format(new Date(link.time), 'MMM d y HH:mm')}
    </span>
    <form action="?/deleteLink" method="POST" use:enhance={confirmDelete}>
      <input type="hidden" name="id" value={link.id} />
      <button type="submit">Delete</button>
    </form>
  </div>
</div>

<style>
  .root {
    font-size: 20px;
  }

  a {
    color: #1111aa;
    text-decoration: none;
  }

  span.tag {
    color: #aa5511;
    display: inline-block;
    margin-right: 10px;
  }

  span.time {
    color: #777777;
  }

  form {
    display: inline-block;
    margin: 0 0 0 8px;
  }

  button {
    position: relative;
    top: -2px;
    background: #f0f0f0;
    border: 1px solid black;
    border-radius: 3px;
    color: #000000;
    font-size: 12px;
    padding: 3px;
  }
</style>
