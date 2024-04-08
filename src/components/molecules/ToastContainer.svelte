<script lang="ts">
  import ToastItem from "$components/atoms/ToastItem.svelte";

  import { dismissToast, toasts } from "$stores";
</script>

{#if $toasts}
  <section
    class="fixed bottom-0 left-0 right-0 z-50 mt-1 flex w-full flex-col justify-center"
  >
    {#each $toasts as toast (toast.id)}
      <ToastItem
        type={toast.type}
        dismissible={toast.dismissible}
        on:dismiss={() => dismissToast(toast.id)}
        >{@html toast.message}</ToastItem
      >
    {/each}
  </section>
{/if}
