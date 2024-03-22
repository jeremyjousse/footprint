<script lang="ts">
  import { colorClasses } from "$components/helpers";
  import type { ColorType } from "$components/types";
  import { twMerge } from "tailwind-merge";

  let node: HTMLLabelElement;

  export let color: ColorType = "gray";
  export let show: boolean = true;
  export let defaultClass: string = "block";

  $: labelClass = twMerge(defaultClass, colorClasses[color], $$props.class);
</script>

{#if show}
  <label bind:this={node} {...$$restProps} class={labelClass}><slot /></label>
{:else}
  <slot />
{/if}
