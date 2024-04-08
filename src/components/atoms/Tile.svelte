<script lang="ts">
  import { twMerge } from "tailwind-merge";

  const bgColors = {
    normal: "bg-light dark:bg-dark-950",
    gray: "bg-gray_scale dark:bg-gray_scale-950",
  };

  const margin = "m-1 sm:m-2";
  const padding = "p-4 sm:p-5";
  const size = "w-full";
  $: tileClass = twMerge(
    margin,
    "flex w-full",
    size,
    "flex-col rounded border shadow-sm",
    padding,
    bgColors["gray"],
    $$props.class
  );
</script>

<div class={tileClass}><slot /></div>
