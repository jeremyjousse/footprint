<script lang="ts">
  import type { TableColorType } from "$components/types";
  import { setContext } from "svelte";
  import { twMerge, twJoin } from "tailwind-merge";
  export let divClass: string = "relative overflow-x-auto";
  export let striped: boolean = true;
  export let hoverable: boolean = true;
  export let bordered: boolean = true;
  export let rounded: boolean = true;
  export let shadowed: boolean = false;
  export let color: TableColorType = "default";
  export let customColor: string = "";

  const colors = {
    default: "text-gray-500 dark:text-gray-400",
    custom: customColor,
  };

  const borderClass: string = bordered
    ? "border border-gray-500 dark:border-gray-400"
    : "";
  const roundedClass: string = rounded ? "rounded" : "";
  const shadowClass: string = shadowed ? "shadow-md sm:rounded-lg" : "";

  $: setContext("striped", striped);
  $: setContext("hoverable", hoverable);
  $: setContext("bordered", bordered);
  $: setContext("color", color);
</script>

<div class={twMerge(divClass, borderClass, roundedClass, shadowClass)}>
  <table
    {...$$restProps}
    class={twMerge(
      "w-full text-left text-sm",
      colors[color],

      $$props.class
    )}
  >
    <slot />
  </table>
</div>
