<script lang="ts">
  export let onclick: () => void;
</script>

<button
  type="button"
  class="rounded-md border border-transparent bg-gray-600 py-2 px-4 mt-4 text-sm font-medium text-white shadow-sm hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
  on:click={onclick}
  ><slot />
</button>
