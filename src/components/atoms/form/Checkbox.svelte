<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import Label from "./Label.svelte";
  import { inputClass } from "$components/helpers";
  import type { ColorType } from "$components/types";
  import { getContext } from "svelte";

  const labelClass = (inline: boolean, extraClass: string) =>
    twMerge(inline ? "inline-flex" : "flex", "items-center", extraClass);

  let background: boolean = getContext("background");

  export let checked: boolean | undefined = undefined;
  export let color: ColorType = "gray";
  export let inline: boolean = false;
  export let spacing: string = $$slots.default ? "me-2" : "";
  export let value: string | number = "on";
</script>

<Label class={labelClass(inline, $$props.class)} show={$$slots.default}>
  <input
    type="checkbox"
    bind:checked
    on:keyup
    on:keydown
    on:keypress
    on:focus
    on:blur
    on:click
    on:mouseover
    on:mouseenter
    on:mouseleave
    on:paste
    on:change
    {value}
    {...$$restProps}
    class={inputClass(
      color,
      true,
      background,
      spacing,
      $$slots.default || $$props.class
    )}
  />
  <slot />
</Label>
