<!-- src/_layout.svelte -->

<script>
  import "../app.css";
  import Navbar from "$components/molecules/Navbar.svelte";
  import appearanceService from "../services/AppearanceService";
  import HomeIcon from "$components/atoms/icons/HomeIcon.svelte";
  import { userLanguage } from "$services";
  import { loadTranslations } from "$i18n";
  import ToastContainer from "$components/molecules/ToastContainer.svelte";

  export let data;

  const loadTranslationsAccordingToUserLocale = async () => {
    const initLocale = await userLanguage();
    await loadTranslations(initLocale, data.pathname);
  };
</script>

<div class="parent-container grid min-h-screen font-sans">
  {#await loadTranslationsAccordingToUserLocale()}
    <p>loading translations</p>
  {:then}
    <div class="w-full">
      <div class="h-full lg:ml-72 xl:ml-80">
        <header
          class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex"
        >
          <div
            class="contents lg:pointer-events-auto lg:block lg:w-52 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 xl:w-52 lg:dark:border-white/10"
          >
            <div class="hidden lg:flex">
              <a href="/"><HomeIcon /></a>
            </div>
            <Navbar />
          </div>
        </header>
        <div class="relative flex h-full flex-col px-4 pt-5 sm:px-6 lg:px-8">
          <slot />
        </div>
      </div>
    </div>
    <ToastContainer />
  {/await}
</div>

<style>
  :global(body) {
    @apply min-h-full antialiased;
  }
</style>
