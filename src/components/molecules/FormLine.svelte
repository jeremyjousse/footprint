<script>
  import Label from "$components/atoms/form/Label.svelte";
  let firstFormElementClass = "col-span-1";
  let secondFormElementClass = "col-span-2";
  let thirdElementClass = "col-span-1";
  if (!$$slots.thirdElement) {
    firstFormElementClass = "col-span-2";
  }
  if (!$$slots.secondFormElement) {
    firstFormElementClass = "col-span-4";
  }
</script>

<div class="grid grid-cols-5 gap-4 my-3">
  <div><slot name="label" /></div>
  <div class={firstFormElementClass}>
    <slot name="firstFormElement" />
  </div>
  {#if $$slots.secondFormElement}
    <div class={secondFormElementClass}>
      <slot name="secondFormElement" />
    </div>
  {/if}
  {#if $$slots.thirdElement}
    <div class={thirdElementClass}>
      <slot name="thirdElement" />
    </div>
  {/if}
</div>
