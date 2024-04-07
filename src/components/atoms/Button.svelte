<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { twMerge } from "tailwind-merge";

  const colorClasses = {
    primary:
      "text-white bg-primary-700 hover:bg-primary-800 dark:bg-primary-600 dark:hover:bg-primary-700",
  };

  export let color: "primary" = "primary";
  export let href: string | undefined = undefined;
  export let tag: "button" | "link" = "button";
  export let type: HTMLButtonAttributes["type"] = "button";

  $: buttonClass = twMerge(
    "rounded-md border border-transparent py-2 px-4 mt-4 text-white shadow-sm text-center font-medium px-5 py-2.5 text-sm",
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
    on:mouseleave
  >
    <slot />
  </a>
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
