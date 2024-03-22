<script lang="ts">
  import type { Sort } from "$domain";
  import SortIcon from "./icons/SortIcon.svelte";
  export let sort: Sort;
  export let column: string;
</script>

<span
  >{#if sort.by === column}<SortIcon order={sort.order} />{/if}</span
>
