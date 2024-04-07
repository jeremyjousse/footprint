<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import { getContext } from "svelte";

  export let tdClass: string = "px-6 py-4 whitespace-nowrap font-medium ";

  let color = "default";
  color = getContext("color");
  $: tdClass = twMerge(
    tdClass,
    color === "default"
      ? "text-gray-900 dark:text-white"
      : "text-blue-50 whitespace-nowrap dark:text-blue-100",
    $$props.class
  );
</script>

<svelte:element
  this={$$props.onclick ? "button" : "td"}
  {...$$restProps}
  class={tdClass}
  on:click
  role={$$props.onclick ? "button" : undefined}
>
  <slot />
</svelte:element>
