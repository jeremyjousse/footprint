<script lang="ts">
  import BreadcrumbItem from "$components/atoms/BreadcrumbItem.svelte";
  import ChevronIcon from "$components/atoms/icons/ChevronIcon.svelte";
  import { text } from "@sveltejs/kit";
  import type { Breadcrumb } from "domain/valueObjects/Breadcrumb";

  export let breadcrumbs: Breadcrumb[];
</script>

<nav class="flex" aria-label="Breadcrumb">
  <ol
    class="inline-flex items-center space-x-1 md:space-x-2 rtl:space-x-reverse"
  >
    {#each breadcrumbs as breadcrumb, index}
      <li class="inline-flex items-center">
        {#if index > 0}
          <ChevronIcon />
        {/if}
        {#if breadcrumb.link}
          <a
            href={breadcrumb.link}
            class="inline-flex items-center text-black hover:text-gray-600 dark:text-gray-400 dark:hover:text-white"
          >
            <BreadcrumbItem text={breadcrumb.text} icon={breadcrumb.icon} />
          </a>
        {:else}
          <BreadcrumbItem text={breadcrumb.text} icon={breadcrumb.icon} />
        {/if}
      </li>
    {/each}
    <!-- 
    <li>
      <div class="flex items-center">
        <svg
          class="rtl:rotate-180 w-3 h-3 text-gray-400 mx-1"
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 6 10"
        >
          <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="m1 9 4-4-4-4"
          />
        </svg>
        <a
          href="#"
          class="ms-1 text-sm font-medium text-gray-700 hover:text-blue-600 md:ms-2 dark:text-gray-400 dark:hover:text-white"
          >Projects</a
        >
      </div>
    </li>
    <li aria-current="page">
      <div class="flex items-center">
        <svg
          class="rtl:rotate-180 w-3 h-3 text-gray-400 mx-1"
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 6 10"
        >
          <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="m1 9 4-4-4-4"
          />
        </svg>
        <span
          class="ms-1 text-sm font-medium text-gray-500 md:ms-2 dark:text-gray-400"
          >Flowbite</span
        >
      </div>
    </li> -->
  </ol>
</nav>
