<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import { getContext } from "svelte";
  import type { TableColorType } from "$components/types";

  export let color: TableColorType = getContext("color");

  const colors = {
    default: "bg-white dark:bg-gray-800 dark:border-gray-700",
    custom: "",
  };

  const hoverColors = {
    default: "hover:bg-gray-50 dark:hover:bg-gray-600",
    custom: "",
  };

  const stripColors = {
    default:
      "odd:bg-white even:bg-gray-50 odd:dark:bg-zinc-800 even:dark:bg-zinc-700",
    custom: "",
  };

  let trClass: string;
  $: trClass = twMerge([
    !getContext("bordered") && "border-b last:border-b-0",
    colors[color],
    getContext("hoverable") && hoverColors[color],
    getContext("striped") && stripColors[color],
    $$props.class,
  ]);
</script>

<tr {...$$restProps} class={trClass} on:click on:contextmenu on:dblclick>
  <slot />
</tr>
