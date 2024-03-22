<script lang="ts">
  import { page } from "$app/stores";

  import { onMount } from "svelte";
  let Icon: any;
  export let text: string;
  export let link: string;
  export let icon: string;

  onMount(async () => {
    Icon = (await import(`./icons/${icon}Icon.svelte`)).default;
  });
</script>

<li
  aria-current={$page.url.pathname === "/" ? "page" : undefined}
  class="rounded text-left p-1 pl-4 {$page.url.pathname.indexOf(link) != -1
    ? 'active'
    : 'non-active'}"
>
  <a class="text-gray-900 dark:text-white" href={link}
    ><svelte:component this={Icon} /> {text}</a
  >
</li>

<style lang="postcss">
  li.active {
    @apply bg-stone-100;
  }

  :global(html.dark) li.active {
    @apply bg-slate-700;
  }
</style>
