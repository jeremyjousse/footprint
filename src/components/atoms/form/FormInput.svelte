<script lang="ts">
  import type { InputConstraint } from "sveltekit-superforms";

  export let value: string | number | null | undefined;
  export let errors: string[] | undefined | { _errors?: string[] } = undefined;
  export let constraints: InputConstraint | undefined = undefined;

  export let type = "text";
  export let name = "text";
  export let placeholder = "";

  //   function handleInput({ target: t }) {
  //     if (type === "number") {
  //       value = t.value === "" ? null : t.valueAsNumber;
  //     } else {
  //       value = t.value;
  //     }
  //   }
  //
</script>

<input
  class="w-full mr-1 p-2 dark:border-gray-600 border rounded"
  {name}
  {placeholder}
  {type}
  bind:value
  aria-invalid={errors ? "true" : undefined}
  {...constraints}
  {...$$restProps}
/>
{#if errors}<span class="text-red-600">{errors}</span>{/if}
