<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "$i18n";
  export let text: string | undefined;
  export let icon: string | undefined;
  let Icon: any | undefined;

  onMount(async () => {
    if (icon) {
      Icon = (await import(`./icons/${icon}Icon.svelte`)).default;
    }
  });
</script>

{#if icon}
  <svelte:component this={Icon} additionalClasses="mr-1" />
{/if}
{#if text}
  <span
    class=" font-medium text-black hover:text-gray-600 dark:text-gray-400 dark:hover:text-white"
    >{$t(text)}</span
  >
{/if}
