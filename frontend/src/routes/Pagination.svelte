<script lang="ts">
  export let page: number
  export let pages: number
  export let url: (page: number) => string
</script>

{#if pages > 1}
  <div>
    {#if page > 1}
      <span>
        <a href={url(1)}>&laquo;</a>
        <a href={url(page - 1)}>&lsaquo;</a>
      </span>
    {/if}
    <span>
      {page}
    </span>
    {#if page < pages}
      <span>
        <a href={url(page + 1)}>&rsaquo;</a>
      </span>
      <span>
        <a href={url(pages)}>&raquo;</a>
      </span>
    {/if}
  </div>
{/if}

<style>
  div {
    display: flex;
    justify-content: center;
  }

  span {
    margin: 0 6px;
  }
</style>
