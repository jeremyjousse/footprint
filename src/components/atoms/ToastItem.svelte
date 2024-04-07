<script lang="ts">
  import { ToastTypes } from "$domain/valueObjects";
  import { createEventDispatcher } from "svelte";
  import { fade } from "svelte/transition";

  import CloseIcon from "$components/atoms/icons/CloseIcon.svelte";

  export let type: ToastTypes = ToastTypes.Info;
  export let dismissible = true;

  const dispatch = createEventDispatcher();
</script>

<article
  class="{type.toString()} mb-1 flex w-80 items-center rounded-md p-1 text-white"
  role="alert"
  transition:fade
>
  <div class="ml-1">
    <slot />
  </div>

  {#if dismissible}
    <button class="m-auto mr-0 text-white" on:click={() => dispatch("dismiss")}>
      <CloseIcon />
    </button>
  {/if}
</article>

<style lang="postcss">
  .error {
    background: IndianRed;
  }
  .success {
    background: MediumSeaGreen;
  }
  .info {
    background: SkyBlue;
  }
  .warning {
    background: DarkOrange;
  }
</style>
