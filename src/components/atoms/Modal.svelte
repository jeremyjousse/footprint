<script lang="ts">
  // issued from https://github.com/themesberg/flowbite-svelte/blob/main/src/lib/modal/Modal.svelte
  import { twMerge } from "tailwind-merge";

  export let open: boolean;
  export let size: string = "sm"; // TODO type

  const sizes = {
    xs: "max-w-md",
    sm: "max-w-lg",
    md: "max-w-2xl",
    lg: "max-w-4xl",
    xl: "max-w-7xl",
  };

  let overlayClass: string =
    "fixed inset-0 z-40 bg-gray_scale-950 bg-opacity-50 dark:bg-opacity-80";

  let modalParentClass: string =
    "fixed top-0 start-0 end-0 md:inset-0 md:h-full z-50 w-full p-4 flex justify-center items-center";

  let modalClass = twMerge(
    "relative flex flex-col mx-auto w-full divide-y rounded border shadow-sm bg-gray_scale-100 dark:bg-gray_scale-950  max-w-2xl w-full max-h-full p-5"
  );

  // TODO check this
  function prepareFocus(node: HTMLElement) {
    const walker = document.createTreeWalker(node, NodeFilter.SHOW_ELEMENT);
    let n: Node | null;
    while ((n = walker.nextNode())) {
      if (n instanceof HTMLElement) {
        const el = n as HTMLElement;
        const [x, y] = isScrollable(el);
        if (x || y) el.tabIndex = 0;
      }
    }
    node.focus();
  }

  //
  // Taken from github.com/carbon-design-system/carbon/packages/react/src/internal/keyboard/navigation.js
  //

  // add all the elements inside modal which you want to make focusable
  // TODO check this
  const selectorTabbable = `
  a[href], area[href], input:not([disabled]):not([tabindex='-1']),
  button:not([disabled]):not([tabindex='-1']),select:not([disabled]):not([tabindex='-1']),
  textarea:not([disabled]):not([tabindex='-1']),
  iframe, object, embed, *[tabindex]:not([tabindex='-1']):not([disabled]), *[contenteditable=true]
`;
  // TODO check this
  function focusTrap(node: any) {
    /** @type {(e:KeyboardEvent)=>void} */
    function handleFocusTrap(e: any) {
      let isTabPressed = e.key === "Tab" || e.keyCode === 9;

      if (!isTabPressed) {
        return;
      }

      const tabbable = Array.from(node.querySelectorAll(selectorTabbable));

      let index = tabbable.indexOf(document.activeElement ?? node);
      if (index === -1 && e.shiftKey) index = 0;
      index += tabbable.length + (e.shiftKey ? -1 : 1);
      index %= tabbable.length;
      /** @ts-ignore */
      tabbable[index].focus();

      e.preventDefault();
    }

    document.addEventListener("keydown", handleFocusTrap, true);

    return {
      destroy() {
        document.removeEventListener("keydown", handleFocusTrap, true);
      },
    };
  }

  // TODO check this
  const isScrollable = (e: HTMLElement): boolean[] => [
    e.scrollWidth > e.clientWidth &&
      ["scroll", "auto"].indexOf(getComputedStyle(e).overflowX) >= 0,
    e.scrollHeight > e.clientHeight &&
      ["scroll", "auto"].indexOf(getComputedStyle(e).overflowY) >= 0,
  ];

  const handleClose = (e: MouseEvent) => {
    const target: Element = e.target as Element;
    // TODO Add button
    if (target.id === "modal-overlay") close(e);
  };

  function handleEscapeKey(e: KeyboardEvent) {
    if (e.key === "Escape") return close(e);
  }

  const close = (e: Event) => {
    e.preventDefault();
    open = false;
  };
</script>

{#if open}
  <!-- backdrop -->
  <div class={overlayClass} />
  <!-- dialog -->
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div
    on:keydown={handleEscapeKey}
    on:wheel|preventDefault|nonpassive
    use:prepareFocus
    use:focusTrap
    on:mousedown={handleClose}
    class={modalParentClass}
    tabindex="-1"
    aria-modal="true"
    role="dialog"
    id="modal-overlay"
  >
    <div class="flex relative {sizes[size]} w-full max-h-full">
      <!-- Modal content -->
      <div class={modalClass}><slot /></div>
    </div>
  </div>
{/if}
