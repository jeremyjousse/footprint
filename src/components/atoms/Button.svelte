<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { twMerge } from "tailwind-merge";

  const colorClasses = {
    primary:
      "text-white bg-primary-700 hover:bg-primary-800 dark:bg-primary-600 dark:hover:bg-primary-700",
    danger:
      "text-white bg-danger-700 hover:bg-danger-800 dark:bg-danger-600 dark:hover:bg-danger-700",
  };

  export let color: "primary" | "danger" = "primary";
  export let href: string | undefined = undefined;
  export let tag: "button" | "link" = "button";
  export let type: HTMLButtonAttributes["type"] = "button";

  $: buttonClass = twMerge(
    "rounded-md border border-transparent py-2 px-4 text-white shadow-sm text-center font-medium text-sm",
    colorClasses[color],
    $$props.disabled && "cursor-not-allowed opacity-50"
  );
</script>

{#if href}
  <a
    {href}
    {...$$restProps}
    class={buttonClass}
    role="button"
    on:click
    on:change
    on:keydown
    on:keyup
    on:touchstart|passive
    on:touchend
    on:touchcancel
    on:mouseenter
    on:mouseleave><slot /></a
  >
{:else if tag === "button"}
  <button
    {type}
    {...$$restProps}
    class={buttonClass}
    on:click
    on:change
    on:keydown
    on:keyup
    on:touchstart|passive
    on:touchend
    on:touchcancel
    on:mouseenter
    on:mouseleave
  >
    <slot />
  </button>
{/if}
