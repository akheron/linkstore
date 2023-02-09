<script lang="ts">
  import { enhance } from '$app/forms'
  import type { ActionData } from './$types'

  export let form: ActionData
</script>

<form method="POST" use:enhance>
  <div>
    <input name="href" placeholder="url" />
  </div>
  <div>
    <input name="description" placeholder="description" />
  </div>
  <div>
    <textarea name="extended" placeholder="long description" />
  </div>
  <div>
    <input name="tags" placeholder="tags" />
  </div>
  <div>
    <button type="submit">save</button>
    <a href="/">cancel</a>
  </div>
</form>
{#if form?.error}
  <div class="error">{form.error}</div>
{/if}

<style>
  .error {
    color: #ff1100;
  }

  form > div {
    margin-bottom: 8px;
  }

  input,
  textarea {
    width: 100%;
  }

  textarea {
    height: 150px;
  }
</style>
