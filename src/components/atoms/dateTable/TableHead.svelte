<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import { getContext } from "svelte";
  import type { TableColorType } from "$components/types";

  export let theadClass: string = "text-xs uppercase";
  export let defaultRow: boolean = true;

  let color: TableColorType = getContext("color");
  let bordered: boolean = getContext("noborder");
  let striped: boolean = getContext("striped");

  let textColor =
    color === "default"
      ? "text-gray-700 dark:text-gray-400"
      : color === "custom"
        ? ""
        : "text-white  dark:text-white";
  let borderColors = striped
    ? ""
    : color === "default"
      ? "border-gray-700"
      : color === "custom"
        ? ""
        : `border-${color}-400`;

  const bgColors = {
    default: !bordered || striped ? "" : "bg-gray-50 dark:bg-gray-700",
    custom: "",
  };

  $: theadClass = twMerge(
    theadClass,
    textColor,
    striped && borderColors,
    bgColors[color],
    $$props.class
  );
</script>

<thead {...$$restProps} class={theadClass}>
  {#if defaultRow}
    <tr>
      <slot />
    </tr>
  {:else}
    <slot />
  {/if}
</thead>
